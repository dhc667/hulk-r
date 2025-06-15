use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::table_builder::{
    GotoTable, TableBuilder,
    items::{LR0Item, LR0ItemSet},
};
use crate::{StateId, SymbolId};

pub struct LR0States {
    pub initial_state: StateId,
    pub initial_production_core: LR0Item,
    pub lr0_states: HashMap<StateId, LR0ItemSet>,
    pub goto_table: GotoTable,
}

impl LR0States {
    pub fn new(
        initial_state: StateId,
        initial_production_core: LR0Item,
        state_items: HashMap<StateId, LR0ItemSet>,
        goto_table: GotoTable,
    ) -> Self {
        Self {
            initial_state,
            initial_production_core,
            lr0_states: state_items,
            goto_table,
        }
    }
}

impl<'b> TableBuilder<'b> {
    pub(super) fn build_lr0_states(&self) -> LR0States {
        let mut id_generator = (0..).map(|i| StateId(i)).into_iter();

        let (first_item, first_set) = self.build_first_lr0_item_set();

        let mut visited: HashSet<LR0ItemSet> = HashSet::new();
        let mut frontier: HashSet<LR0ItemSet> = HashSet::new();
        let mut id_map: HashMap<LR0ItemSet, StateId> = HashMap::new();
        let mut goto_table = GotoTable::new();

        let first_set_id = id_generator.next().unwrap();

        id_map.insert(first_set.clone(), first_set_id);
        frontier.insert(first_set);

        loop {
            let mut next_frontier = HashSet::new();

            for item_set in frontier.iter() {
                let goto_kernels = self.get_goto_kernels(&item_set);
                for (symbol, kernel) in goto_kernels {
                    let new_set = self.lr0_closure(kernel);
                    let new_set = LR0ItemSet::from_iter(new_set.into_iter());

                    let set_id = if !visited.contains(&new_set)
                        && !next_frontier.contains(&new_set)
                        && !frontier.contains(&new_set)
                    {
                        let set_id = id_generator.next().unwrap();
                        id_map.insert(new_set.clone(), set_id);
                        next_frontier.insert(new_set);

                        set_id
                    } else {
                        *id_map.get(&new_set).unwrap()
                    };

                    let from_set_id = id_map.get(&item_set).unwrap();
                    goto_table.insert((*from_set_id, symbol), set_id);
                }
            }

            for item_set in frontier {
                visited.insert(item_set);
            }

            if next_frontier.is_empty() {
                break;
            }

            frontier = next_frontier;
        }

        LR0States::new(
            first_set_id,
            first_item,
            Self::swap_key_values(id_map),
            goto_table,
        )
    }

    fn build_first_lr0_item_set(&self) -> (LR0Item, LR0ItemSet) {
        let first_item = LR0Item::new(self.first_production_id, 0);

        let mut first_item_set = HashSet::new();
        first_item_set.insert(first_item);
        first_item_set = self.lr0_closure(first_item_set);

        (first_item, LR0ItemSet::from_iter(first_item_set))
    }

    fn get_goto_kernels(&self, item_set: &LR0ItemSet) -> HashMap<SymbolId, HashSet<LR0Item>> {
        let mut goto_symbols = HashMap::new();

        for item in item_set.iter() {
            let symbol = self.symbol_right_of_dot(item);
            if symbol.is_none() {
                continue;
            }
            let symbol = symbol.unwrap();

            let symbol_items = {
                if goto_symbols.get(&symbol).is_none() {
                    goto_symbols.insert(symbol, HashSet::new());
                }

                goto_symbols.get_mut(&symbol).unwrap()
            };

            symbol_items.insert(self.increased_dot_position(*item).unwrap());
        }

        goto_symbols
    }

    fn swap_key_values<K: Eq + Hash, V: Eq + Hash>(x: HashMap<K, V>) -> HashMap<V, K> {
        x.into_iter().map(|(k, v)| (v, k)).collect()
    }
}
