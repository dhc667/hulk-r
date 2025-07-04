use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use crate::{
    NonTerminalId, Parser, ProductionCompute, TerminalCompute, TerminalId, Token,
    debugging_helpers::get_name_or_default, table_builder::TableBuilder,
};
use crate::{Production, ProductionId, SymbolId};

pub struct Grammar<TokenType: Eq + Hash + Copy + Debug, R> {
    symbol_id_generator: std::ops::RangeFrom<usize>,
    production_id_generator: std::ops::RangeFrom<usize>,

    symbols: HashMap<SymbolId, Option<String>>,
    name_to_symbol: HashMap<String, SymbolId>,

    productions: HashMap<ProductionId, Production>,
    production_computes: HashMap<ProductionId, ProductionCompute<R>>,
    terminal_computes: HashMap<TerminalId, TerminalCompute<TokenType, R>>,
    token_type_to_terminal: HashMap<TokenType, TerminalId>,

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
        let mut id_generator = (0..).into_iter();

        let mut definer = Self {
            symbols: HashMap::new(),
            name_to_symbol: HashMap::new(),

            productions: HashMap::new(),
            production_computes: HashMap::new(),
            terminal_computes: HashMap::new(),
            token_type_to_terminal: HashMap::new(),

            first_symbol: None,

            epsilon: TerminalId::new(id_generator.next().unwrap()),
            eof: TerminalId::new(id_generator.next().unwrap()),
            extra_symbol: TerminalId::new(id_generator.next().unwrap()),
            symbol_id_generator: id_generator,
            production_id_generator: (0..).into_iter(),
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

        if self.token_type_to_terminal.contains_key(&token_type) {
            return Err("Terminal for the given token type is already defined".to_string());
        }

        let terminal = TerminalId::new(self.generate_symbol_id());
        self.terminal_computes.insert(terminal, Box::new(compute));
        self.token_type_to_terminal.insert(token_type, terminal);

        let symbol = SymbolId::from(terminal);

        self.symbols.insert(symbol, name.clone());
        if let Some(name) = name {
            self.name_to_symbol.insert(name, symbol);
        }

        Ok(symbol)
    }

    pub fn define_non_terminal(&mut self, name: Option<String>) -> Result<SymbolId, String> {
        self.check_if_defined_name(&name)?;

        let smbol = SymbolId::from(NonTerminalId::new(self.generate_symbol_id()));

        self.symbols.insert(smbol, name.clone());

        if let Some(name) = name {
            self.name_to_symbol.insert(name, smbol);
        }

        Ok(smbol)
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
            self.first_symbol = Some(NonTerminalId::new(self.generate_symbol_id()));

            let symbol = SymbolId::from(self.first_symbol.unwrap());
            self.symbols.insert(symbol, name.clone());

            if let Some(name) = name {
                self.name_to_symbol.insert(name, symbol);
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
    ) -> ProductionId {
        let id = ProductionId::new(self.production_id_generator.next().unwrap());

        self.productions.insert(id, Production::new(lhs, rhs));

        self.production_computes.insert(id, Box::new(compute));

        id
    }

    pub fn build_parser(mut self) -> Result<Parser<TokenType, R>, Vec<String>> {
        self.check_if_all_non_terminals_have_productions()?;
        let augmented_first_production_id = self.augment()?;

        let (action_table, goto_table) = TableBuilder::build_tables(
            &self.symbols,
            &self.productions,
            augmented_first_production_id,
            self.epsilon,
            self.eof,
            self.extra_symbol,
        )?;

        Ok(Parser::new(
            action_table,
            self.symbols,
            self.production_computes,
            self.productions,
            self.terminal_computes,
            self.token_type_to_terminal,
            goto_table,
            self.eof,
        ))
    }

    fn check_if_all_non_terminals_have_productions(&self) -> Result<(), Vec<String>> {
        let mut dangling_non_terminals = self
            .symbols
            .iter()
            .map(|(id, _)| *id)
            .filter(|id| id.is_non_terminal_id())
            .map(|id| *id.as_non_terminal_id().unwrap())
            .collect::<HashSet<NonTerminalId>>();

        self.productions.iter().for_each(|(_, p)| {
            dangling_non_terminals.remove(&p.lhs);
        });

        let errors = dangling_non_terminals
            .iter()
            .map(|id| get_name_or_default(&SymbolId::from(*id), &self.symbols))
            .map(|name| format!("Non terminal {} has no productions associated to it", name))
            .collect::<Vec<String>>();

        match errors.len() > 0 {
            true => Err(errors),
            false => Ok(()),
        }
    }

    // creates a first production S' -> S where S was the first symbol, returns the index of the
    // created production in the production vector
    fn augment(&mut self) -> Result<ProductionId, Vec<String>> {
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

        let new_first_symbol = SymbolId::from(NonTerminalId::new(self.generate_symbol_id()));

        self.symbols
            .insert(new_first_symbol, Some(first_symbol_name + "'"));

        Ok(self.production(
            *new_first_symbol.as_non_terminal_id().unwrap(),
            vec![SymbolId::from(self.first_symbol.unwrap())],
            |mut val_vec| val_vec.pop().unwrap(),
        ))
    }

    fn is_first_symbol_defined(&self) -> bool {
        self.first_symbol != None
    }

    fn generate_symbol_id(&mut self) -> usize {
        self.symbol_id_generator.next().unwrap()
    }
}
