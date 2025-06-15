use std::collections::{HashMap, HashSet};

use crate::table_builder::{
    GotoTable, TableBuilder,
    conflicts::{Conflict, ReduceReduceConflict, ShiftReduceConflict},
    items::{LR0Item, LR1Item},
};
use crate::{
    StateId, SymbolId, TerminalId, debugging_helpers::get_name_or_default, parser::Action,
};

impl<'b> TableBuilder<'b> {
    pub(super) fn build_action_table(
        &self,
        initial_state: StateId,
        initial_production_core: LR0Item,
        lr1_item_sets: &HashMap<StateId, HashSet<LR1Item>>,
        goto_table: &GotoTable,
    ) -> Result<HashMap<(StateId, TerminalId), Action>, Vec<Conflict>> {
        let mut conflicts: Vec<Conflict> = Vec::new();
        let mut action_table: HashMap<(StateId, TerminalId), Action> = HashMap::new();

        let final_state = goto_table
            .get(&(
                initial_state,
                self.symbol_right_of_dot(&initial_production_core).unwrap(),
            ))
            .unwrap();

        for (state_id, item_set) in lr1_item_sets.iter() {
            for item in item_set.iter() {
                match self.symbol_right_of_dot(&item.core()) {
                    None => {
                        for f in item.follow().iter() {
                            let action_key = (*state_id, *f);
                            let action_to_add = Action::Reduce(*item.core().production_id());
                            let conflict =
                                self.add_to_table(action_key, action_to_add, *f, &mut action_table);

                            if conflict.is_some() {
                                conflicts.push(conflict.unwrap());
                            }
                        }
                    }
                    Some(symbol) => {
                        if symbol.is_terminal_id() {
                            let f = symbol.as_terminal_id().unwrap();
                            let action_key = (*state_id, *f);
                            let goto_key = (*state_id, symbol);
                            let action_to_add = Action::Shift(*goto_table.get(&goto_key).unwrap());

                            let conflict =
                                self.add_to_table(action_key, action_to_add, *f, &mut action_table);
                            if conflict.is_some() {
                                conflicts.push(conflict.unwrap());
                            }
                        }
                    }
                }
            }
        }

        action_table.insert((*final_state, self.eof), Action::Accept);

        if conflicts.len() > 0 {
            Err(conflicts)
        } else {
            Ok(action_table)
        }
    }

    fn add_to_table(
        &self,
        action_key: (StateId, TerminalId),
        action_to_add: Action,
        f: TerminalId,
        action_table: &mut HashMap<(StateId, TerminalId), Action>,
    ) -> Option<Conflict> {
        if action_table.contains_key(&action_key) {
            let action = action_table.get(&action_key).unwrap();
            if action == &action_to_add {
                return None;
            }

            return Some(self.create_conflict(
                *action_table.get(&action_key).unwrap(),
                action_to_add,
                f,
            ));
        } else {
            action_table.insert(action_key, action_to_add);
            None
        }
    }

    fn create_conflict(&self, action_1: Action, action_2: Action, follow: TerminalId) -> Conflict {
        match (action_1, action_2) {
            (Action::Accept, _) | (_, Action::Accept) => {
                panic!("Accept cannot be part of a conflict")
            }
            (Action::Shift(_), Action::Reduce(production_id)) => Conflict::ShiftReduce(
                ShiftReduceConflict::new(follow, self.get_production_symbols(production_id)),
            ),
            (Action::Shift(s1), Action::Shift(s2)) => {
                panic!(
                    "There are no shift/shift conflicts ({}/{}, {})",
                    s1.0,
                    s2.0,
                    get_name_or_default(&SymbolId::from(follow), &self.symbols)
                )
            }
            (Action::Reduce(production_id_1), Action::Reduce(production_id_2)) => {
                Conflict::ReduceReduce(ReduceReduceConflict::new(
                    follow,
                    self.get_production_symbols(production_id_1),
                    self.get_production_symbols(production_id_2),
                ))
            }
            (a, b) => self.create_conflict(b, a, follow),
        }
    }
}
