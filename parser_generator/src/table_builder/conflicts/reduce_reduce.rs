use std::collections::HashMap;

use crate::{
    SymbolId, TerminalId,
    debugging_helpers::{get_name_or_default, get_production_string},
};

pub struct ReduceReduceConflict {
    pub follow: TerminalId,
    pub reduce_production_1: (SymbolId, Vec<SymbolId>),
    pub reduce_production_2: (SymbolId, Vec<SymbolId>),
}

impl ReduceReduceConflict {
    pub fn new(
        follow: TerminalId,
        reduce_production_1: (SymbolId, Vec<SymbolId>),
        reduce_production_2: (SymbolId, Vec<SymbolId>),
    ) -> Self {
        Self {
            follow,
            reduce_production_1,
            reduce_production_2,
        }
    }

    pub fn to_string(&self, symbol_names: &HashMap<SymbolId, Option<String>>) -> String {
        let (reduce_production_1_lhs_name, reduce_production_1_rhs) =
            get_production_string(&self.reduce_production_1, symbol_names);

        let reduce_production_1 = reduce_production_1_lhs_name + " -> " + &reduce_production_1_rhs;

        let (reduce_production_2_lhs_name, reduce_production_2_rhs) =
            get_production_string(&self.reduce_production_2, symbol_names);

        let reduce_production_2 = reduce_production_2_lhs_name + " -> " + &reduce_production_2_rhs;

        let follow_symbol = SymbolId::from(self.follow);
        let follow_name = get_name_or_default(&follow_symbol, symbol_names);

        format!("The parser can reach a state such that:\n")
            + &format!("{reduce_production_1_rhs}\n")
            + &format!("as well as\n")
            + &format!("{reduce_production_2_rhs}\n")
            + &format!("Are on top of the stack\n")
            + &format!("And the lookahead is {follow_name}\n")
            + &format!("The parser can either reduce by:\n")
            + &format!("{reduce_production_1}\n")
            + &format!("or by\n")
            + &format!("{reduce_production_2}\n")
    }
}
