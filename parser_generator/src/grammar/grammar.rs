use std::{
    collections::{HashMap, HashSet}, fmt::Debug, hash::Hash
};

use crate::{
    Parser, Production, Token,
    parser_generator::{ParserGenerator, get_name_or_default},
    symbol::{NonTerminalId, SymbolId, Terminal, TerminalId},
};

pub struct Grammar<TokenType: Eq + Hash + Copy, R> {
    last_symbol_id: usize,
    symbols: HashMap<SymbolId, Option<String>>,
    name_to_symbol: HashMap<String, SymbolId>,
    productions: Vec<Production<R>>,
    terminals: HashMap<TokenType, Terminal<TokenType, R>>,
    first_symbol: Option<NonTerminalId>,
    /// symbol that represents no terminal, empty
    epsilon: TerminalId,
    /// $, end of input
    eof: TerminalId,
    /// symbol that does not appear in the grammar, useful in generation
    extra_symbol: TerminalId,
}

impl<'a, TokenType: Eq + Hash + Copy + Debug, R> Grammar<TokenType, R> {
    pub fn new() -> Self {
        let mut definer = Self {
            epsilon: TerminalId(0),
            eof: TerminalId(1),
            extra_symbol: TerminalId(2),
            last_symbol_id: 2,
            symbols: HashMap::new(),
            name_to_symbol: HashMap::new(),
            productions: Vec::new(),
            terminals: HashMap::new(),
            first_symbol: None,
        };

        let epsilon = "__e__".to_string();
        let eof = "__$__".to_string();
        let extra = "__#__".to_string();

        for (name, id) in vec![
            (epsilon, definer.epsilon),
            (eof, definer.eof),
            (extra, definer.extra_symbol),
        ] {
            let id = SymbolId::from(id);
            definer.symbols.insert(id, Some(name.clone()));
            definer.name_to_symbol.insert(name, id);
        }

        definer
    }

    pub fn define_terminal(
        &mut self,
        token_type: TokenType,
        compute: impl Fn(&Token<TokenType>) -> R + 'static,
        name: Option<String>,
    ) -> Result<SymbolId, String> {
        self.check_if_defined_name(&name)?;

        if self.terminals.contains_key(&token_type) {
            return Err("Terminal for the given token type is already defined".to_string());
        }

        let terminal = Terminal::new(self.generate_symbol_id(), token_type, Box::new(compute));
        let terminal_id = terminal.id;

        self.terminals.insert(token_type, terminal);
        let symbol_id = SymbolId::from(terminal_id);
        self.symbols.insert(symbol_id, name.clone());
        if let Some(name) = name {
            self.name_to_symbol.insert(name, symbol_id);
        }

        Ok(SymbolId::from(terminal_id))
    }

    pub fn define_non_terminal(&mut self, name: Option<String>) -> Result<SymbolId, String> {
        self.check_if_defined_name(&name)?;

        let symbol_id = SymbolId::from(NonTerminalId(self.generate_symbol_id()));

        self.symbols.insert(symbol_id, name.clone());

        if let Some(name) = name {
            self.name_to_symbol.insert(name, symbol_id);
        }

        Ok(symbol_id)
    }

    fn check_if_defined_name(&self, name: &Option<String>) -> Result<(), String> {
        if let Some(name) = name.as_ref() {
            if self.name_to_symbol.contains_key(name) {
                return Err(format!("Symbol with name {name} is already defined"));
            }
        }

        Ok(())
    }

    pub fn define_first_symbol(&mut self, name: Option<String>) -> Result<NonTerminalId, String> {
        if self.is_first_symbol_defined() {
            Err("A first symbol has already been defined".to_string())
        } else {
            self.first_symbol = Some(NonTerminalId(self.generate_symbol_id()));
            let symbol_id = SymbolId::from(self.first_symbol.unwrap());
            self.symbols.insert(symbol_id, name.clone());
            if let Some(name) = name {
                self.name_to_symbol.insert(name, symbol_id);
            }
            Ok(self.first_symbol.unwrap())
        }
    }

    pub fn get_or_create_non_terminal(&mut self, name: String) -> SymbolId {
        if !self.name_to_symbol.contains_key(&name) {
            let symbol = self.define_non_terminal(Some(name)).unwrap();
            symbol
        } else {
            self.get_symbol_by_name(&name).unwrap()
        }
    }

    pub fn get_symbol_by_name(&self, name: &str) -> Option<SymbolId> {
        self.name_to_symbol.get(name).map(|s| *s)
    }

    pub fn production(
        &mut self,
        lhs: NonTerminalId,
        rhs: Vec<SymbolId>,
        compute: impl Fn(Vec<R>) -> R + 'static,
    ) {
        self.productions.push(Production::new(lhs, rhs, compute));
    }

    pub fn epsilon(&self) -> SymbolId {
        SymbolId::from(self.epsilon)
    }

    pub fn build_parser(mut self) -> Result<Parser<TokenType, R>, Vec<String>> {
        self.check_if_all_non_terminals_have_productions()?;
        let augmented_first_production_index = self.augment()?;

        let terminals: HashMap<TerminalId, Terminal<TokenType, R>> = self
            .terminals
            .into_iter()
            .map(|(_, terminal)| (terminal.id, terminal))
            .collect();

        let symbol_table = terminals
            .iter()
            .map(|(_, terminal)| (terminal.token_type, terminal.id))
            .collect();

        ParserGenerator::build_parser(
            self.symbols,
            self.productions,
            augmented_first_production_index,
            terminals,
            symbol_table,
            self.epsilon,
            self.eof,
            self.extra_symbol,
        )
    }

    fn check_if_all_non_terminals_have_productions(&self) -> Result<(), Vec<String>> {
        let mut dangling_non_terminals = self
            .symbols
            .iter()
            .map(|(id, _)| *id)
            .filter(|id| id.is_non_terminal_id())
            .map(|id| *id.as_non_terminal_id().unwrap())
            .collect::<HashSet<NonTerminalId>>();

        self.productions.iter().for_each(|p| {
            dangling_non_terminals.remove(&p.lhs);
        });

        let errors = dangling_non_terminals
            .iter()
            .map(|id| get_name_or_default(&SymbolId::from(*id), &self.symbols))
            .map(|name| format!("Non terminal {} has no productions associated to it", name))
            .collect::<Vec<String>>();

        match errors.len() > 0 {
            true => {
                Err(errors)
            },
            false => {
                Ok(())
            }
        }
    }

    // creates a first production S' -> S where S was the first symbol, returns the index of the
    // created production in the production vector
    fn augment(&mut self) -> Result<usize, Vec<String>> {
        if !self.is_first_symbol_defined() {
            return Err(vec![
                "Cannot build a parser with a grammar without an initial symbol".to_string(),
            ]);
        }
        let first_symbol = SymbolId::from(self.first_symbol.unwrap());
        let first_symbol_name = self
            .symbols
            .get(&first_symbol)
            .unwrap()
            .as_ref()
            .unwrap_or(&first_symbol.id_string())
            .to_string();

        let new_first_symbol = SymbolId::from(NonTerminalId(self.generate_symbol_id()));

        self.symbols
            .insert(new_first_symbol, Some(first_symbol_name + "'"));

        self.production(
            *new_first_symbol.as_non_terminal_id().unwrap(),
            vec![SymbolId::from(self.first_symbol.unwrap())],
            |mut val_vec| val_vec.pop().unwrap(),
        );

        Ok(self.productions.len() - 1)
    }

    fn is_first_symbol_defined(&self) -> bool {
        self.first_symbol != None
    }

    fn generate_symbol_id(&mut self) -> usize {
        self.last_symbol_id += 1;
        return self.last_symbol_id;
    }
}
