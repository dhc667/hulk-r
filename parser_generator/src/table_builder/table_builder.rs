use std::collections::{HashMap, HashSet};

use crate::{
    NonTerminalId, Production, ProductionId, StateId, SymbolId, TerminalId, parser::Action,
    table_builder::conflicts::Conflict,
};

pub type ActionTable = HashMap<(StateId, TerminalId), Action>;
pub type GotoTable = HashMap<(StateId, SymbolId), StateId>;

pub(crate) struct TableBuilder<'a> {
    pub(super) symbols: &'a HashMap<SymbolId, Option<String>>,
    pub(super) productions: &'a HashMap<ProductionId, Production>,
    pub(super) first_production_id: ProductionId,

    pub(super) epsilon: TerminalId,
    pub(super) eof: TerminalId,
    pub(super) extra_symbol: TerminalId,

    pub(super) first: HashMap<SymbolId, HashSet<TerminalId>>,
    pub(super) action_table: ActionTable,
    pub(super) goto: GotoTable,
}

impl<'a> TableBuilder<'a> {
    pub(crate) fn build_tables(
        symbols: &'a HashMap<SymbolId, Option<String>>,
        productions: &'a HashMap<ProductionId, Production>,
        first_production_id: ProductionId,
        epsilon: TerminalId,
        eof: TerminalId,
        extra_symbol: TerminalId,
    ) -> Result<(ActionTable, GotoTable), Vec<String>> {
        let mut generator = Self {
            symbols,
            productions,
            first_production_id,

            epsilon,
            eof,
            extra_symbol,

            action_table: HashMap::new(),
            goto: HashMap::new(),

            first: HashMap::new(),
        };

        generator.build().map_err(|v| {
            v.into_iter()
                .map(|conflict| conflict.to_string(&generator.symbols))
                .collect::<Vec<String>>()
        })?;

        Ok((generator.action_table, generator.goto))
    }

    fn build(&mut self) -> Result<(), Vec<Conflict>> {
        self.compute_first_for_all_symbols();
        self.build_parsing_tables()
    }

    pub(super) fn productions_for_symbol(
        non_terminal_id: NonTerminalId,
        productions: &HashMap<ProductionId, Production>,
    ) -> impl Iterator<Item = (&ProductionId, &Production)> {
        productions
            .iter()
            .filter(move |(_, prod)| prod.lhs == non_terminal_id)
    }

    pub(super) fn get_production_symbols(
        &self,
        production_id: ProductionId,
    ) -> (SymbolId, Vec<SymbolId>) {
        let prod = self.productions.get(&production_id).unwrap();

        (
            SymbolId::from(prod.lhs),
            prod.rhs.iter().map(|s| *s).collect(),
        )
    }
}
