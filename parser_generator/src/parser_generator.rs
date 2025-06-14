mod closure;
mod first;
mod goto;
mod item;
mod parsing_table;

use std::{
    collections::{HashMap, HashSet}, fmt::Debug, hash::Hash
};

use crate::{
    Parser, Production, SymbolId,
    parser::{Action, ProductionId, StateId},
    parser_generator::parsing_table::Conflict,
    symbol::{NonTerminalId, Terminal, TerminalId},
};

pub(crate) use parsing_table::get_name_or_default;

pub(crate) struct ParserGenerator<TokenType: Eq + Hash + Copy + Debug, R> {
    symbols: HashMap<SymbolId, Option<String>>,
    productions: HashMap<ProductionId, Production<R>>,
    // token_types: HashSet<TokenType>,
    // augmented_first_production: Production<R>,
    first_production_id: ProductionId,
    terminals: HashMap<TerminalId, Terminal<TokenType, R>>,
    symbol_table: HashMap<TokenType, TerminalId>,

    epsilon: TerminalId,
    eof: TerminalId,
    extra_symbol: TerminalId,

    first: HashMap<SymbolId, HashSet<TerminalId>>,
    // follow: HashMap<SymbolId, HashSet<TerminalId>>,
    action_table: HashMap<(StateId, TerminalId), Action>,
    goto: HashMap<(StateId, SymbolId), StateId>,
}

impl<'a, TokenType: Eq + Hash + Copy + Debug, R> ParserGenerator<TokenType, R> {
    pub(crate) fn build_parser(
        symbols: HashMap<SymbolId, Option<String>>,
        productions: Vec<Production<R>>,
        first_production_index: usize,
        terminals: HashMap<TerminalId, Terminal<TokenType, R>>,
        symbol_table: HashMap<TokenType, TerminalId>,
        epsilon: TerminalId,
        eof: TerminalId,
        extra_symbol: TerminalId,
    ) -> Result<Parser<TokenType, R>, Vec<String>> {
        let productions_length = productions.len();
        let productions = productions
            .into_iter()
            .zip(0..productions_length)
            .map(|(p, i)| (ProductionId(i), p))
            .collect();

        let mut generator = Self {
            symbols,
            productions,
            // token_types,
            terminals,
            // augmented_first_production,
            first_production_id: ProductionId(first_production_index),
            symbol_table,

            epsilon,
            eof,
            extra_symbol,

            action_table: HashMap::new(),
            goto: HashMap::new(),

            // follow: HashMap::new(),
            first: HashMap::new(),
        };

        generator.build().map_err(|v| {
            v.into_iter()
                .map(|conflict| conflict.to_string(&generator.symbols))
                .collect::<Vec<String>>()
        })?;

        Ok(Parser::new(
            generator.action_table,
            generator.productions,
            generator.terminals,
            generator.symbol_table,
            generator.symbols,
            generator.goto,
            generator.eof,
        ))
    }

    fn build(&mut self) -> Result<(), Vec<Conflict>> {
        self.compute_first_for_all_symbols();

        self.build_parsing_tables()
    }

    fn productions_for_symbol(
        non_terminal_id: NonTerminalId,
        productions: &HashMap<ProductionId, Production<R>>,
    ) -> impl Iterator<Item = (&ProductionId, &Production<R>)> {
        productions
            .iter()
            .filter(move |(_, prod)| prod.lhs == non_terminal_id)
    }
}
