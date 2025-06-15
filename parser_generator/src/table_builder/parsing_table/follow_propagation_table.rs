use std::collections::{BTreeSet, HashMap, HashSet};

use crate::StateId;
use crate::table_builder::{
    GotoTable, TableBuilder,
    items::{LR0Item, LR1Item, LR1ItemSetBuilder},
};

type LR0ItemSet = BTreeSet<LR0Item>;
type PropagationTable = HashMap<(StateId, LR0Item), HashSet<(StateId, LR0Item)>>;

impl<'b> TableBuilder<'b> {
    pub fn build_lr1_states(
        &self,
        initial_state: &StateId,
        initial_production_core: &LR0Item,
        lr0_states: HashMap<StateId, LR0ItemSet>,
        goto_table: &GotoTable,
    ) -> HashMap<StateId, HashSet<LR1Item>> {
        let lr0_states_kernels = lr0_states
            .into_iter()
            .map(|(id, s)| (id, self.kernel(s)))
            .collect();

        let (mut spontaneous_propagations, propagation_table) =
            self.build_spontaneous_propagation_table(lr0_states_kernels, &goto_table);

        self.add_eof_to_initial_production(
            initial_state,
            initial_production_core,
            &mut spontaneous_propagations,
        );

        // self.dbg_states_builders(&spontaneous_propagations);
        // self.dbg_propagation_table(&propagation_table);

        let lr1_kernels = self.propagate_follows(spontaneous_propagations, propagation_table);

        // self.dbg_states_builders(&lr1_kernels);

        let lr1_states: HashMap<StateId, HashSet<LR1Item>> = lr1_kernels
            .into_iter()
            .map(|(state_id, kernel)| (state_id, self.lr1_closure(kernel.to_hash_set().unwrap())))
            .collect();

        lr1_states
    }

    fn build_spontaneous_propagation_table(
        &self,
        lr0_states_kernels: HashMap<StateId, LR0ItemSet>,
        goto_table: &GotoTable,
    ) -> (HashMap<StateId, LR1ItemSetBuilder>, PropagationTable) {
        let mut spontaneous_propagations = Self::initialize_propagation_table(&lr0_states_kernels);

        let mut propagation_table = PropagationTable::new();

        // for each (id, set) in lr0 states
        for (i, set) in lr0_states_kernels {
            // for each kernel item k
            for k in self.non_final_items(&set) {
                // c = closure([k, #])
                let numeral_item =
                    LR1Item::build(*k, vec![self.extra_symbol].into_iter().collect()).unwrap();

                // self.dbg_lr1_item(&numeral_item);

                let closure = self.lr1_closure(vec![numeral_item].into_iter().collect());

                // self.dbg_lr1_item_set(&closure);

                // for each i = (core, follow) = [A -> p.Xs, follow] in c
                for item in closure.iter() {
                    let (core, follow) = (item.core(), item.follow());
                    if self.get_production_symbols(*core.production_id()).1.len() == 0 {
                        continue;
                    }
                    // j_id = gototable[id, X]
                    let x = self.symbol_right_of_dot(&core).unwrap();
                    let j = goto_table.get(&(i, x)).unwrap();
                    // core_j = core.increased_index()
                    let core = self.increased_dot_position(*core).unwrap();

                    // if follow contains #
                    if follow.contains(&self.extra_symbol) {
                        // PropagationTable.insert((id, k), (j_id, core_j))
                        if !propagation_table.contains_key(&(i, *k)) {
                            propagation_table.insert((i, *k), HashSet::new());
                        }
                        propagation_table
                            .get_mut(&(i, *k))
                            .unwrap()
                            .insert((*j, core));
                    }

                    // J[core_j].insert(follow - {#})
                    spontaneous_propagations
                        .get_mut(j)
                        .unwrap()
                        .insert_to_core_if_exists(
                            &core,
                            &follow
                                .iter()
                                .map(|f| *f)
                                .filter(|f| *f != self.extra_symbol)
                                .collect(),
                        );
                }
            }
        }

        (spontaneous_propagations, propagation_table)
    }

    fn add_eof_to_initial_production(
        &self,
        initial_state: &StateId,
        initial_production_core: &LR0Item,
        spontaneous_propagations: &mut HashMap<StateId, LR1ItemSetBuilder>,
    ) {
        spontaneous_propagations
            .get_mut(initial_state)
            .unwrap()
            .insert_to_core_if_exists(
                initial_production_core,
                &vec![self.eof].into_iter().collect(),
            );
    }

    fn propagate_follows(
        &self,
        mut spontaneous_propagations: HashMap<StateId, LR1ItemSetBuilder>,
        propagation_table: PropagationTable,
    ) -> HashMap<StateId, LR1ItemSetBuilder> {
        let state_ids: Vec<StateId> = spontaneous_propagations.iter().map(|(id, _)| *id).collect();
        loop {
            let mut changed = false;

            for i_id in state_ids.iter() {
                let i = spontaneous_propagations.remove(i_id).unwrap();

                for (i_core, follow) in i.iter() {
                    if !propagation_table.contains_key(&(*i_id, *i_core)) {
                        continue;
                    }
                    for (j_id, j_core) in propagation_table.get(&(*i_id, *i_core)).unwrap() {
                        if j_id == i_id {
                            continue;
                        }

                        changed |= spontaneous_propagations
                            .get_mut(j_id)
                            .unwrap()
                            .insert_to_core_if_exists(j_core, follow)
                    }
                }

                spontaneous_propagations.insert(*i_id, i);
            }

            if !changed {
                break;
            }
        }

        spontaneous_propagations
    }

    fn initialize_propagation_table(
        lr0_states: &HashMap<StateId, LR0ItemSet>,
    ) -> HashMap<StateId, LR1ItemSetBuilder> {
        lr0_states
            .iter()
            .map(|(state_id, lr0_item_set)| {
                (
                    *state_id,
                    LR1ItemSetBuilder::from(
                        lr0_item_set
                            .iter()
                            .map(|i| *i)
                            .collect::<HashSet<LR0Item>>(),
                    ),
                )
            })
            .collect()
    }

    fn non_final_items<'a>(
        &self,
        lr0_item_set: &'a LR0ItemSet,
    ) -> impl Iterator<Item = &'a LR0Item> {
        lr0_item_set
            .iter()
            .filter(|item| self.symbol_right_of_dot(item).is_some())
    }
}
