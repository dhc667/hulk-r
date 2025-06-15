use std::collections::HashMap;

use crate::{
    SymbolId,
    table_builder::conflicts::{ReduceReduceConflict, ShiftReduceConflict},
};

pub enum Conflict {
    ShiftReduce(ShiftReduceConflict),
    ReduceReduce(ReduceReduceConflict),
}

impl Conflict {
    pub fn to_string(&self, symbol_names: &HashMap<SymbolId, Option<String>>) -> String {
        match self {
            Conflict::ShiftReduce(shift_reduce_conflict) => {
                shift_reduce_conflict.to_string(symbol_names)
            }
            Conflict::ReduceReduce(reduce_reduce_conflict) => {
                reduce_reduce_conflict.to_string(symbol_names)
            }
        }
    }
}
