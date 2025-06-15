use std::collections::HashMap;

use crate::{
    SymbolId, TerminalId,
    debugging_helpers::{get_name_or_default, get_production_string},
};

pub struct ShiftReduceConflict {
    pub follow: TerminalId,
    pub reduce_production: (SymbolId, Vec<SymbolId>),
}

impl ShiftReduceConflict {
    pub fn new(follow: TerminalId, reduce_production: (SymbolId, Vec<SymbolId>)) -> Self {
        Self {
            follow,
            reduce_production,
        }
    }

    pub fn to_string(&self, symbol_names: &HashMap<SymbolId, Option<String>>) -> String {
        let (reduce_production_lhs_name, reduce_production_rhs) =
            get_production_string(&self.reduce_production, symbol_names);

        let reduce_production = reduce_production_lhs_name + " -> " + &reduce_production_rhs;

        let follow_symbol = SymbolId::from(self.follow);
        let follow_name = get_name_or_default(&follow_symbol, symbol_names);

        format!("When the following symbols are on top of the stack:\n")
            + &format!("{reduce_production_rhs}\n")
            + &format!("And the lookahead is {follow_name}\n")
            + &format!("The parser can either reduce by:\n")
            + &format!("{reduce_production}\n")
            + &format!("Or shift {follow_name}")
    }
}
