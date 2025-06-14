use std::{
    collections::{HashMap, HashSet}, fmt::Debug, hash::Hash
};

use crate::{
    SymbolId,
    parser::{Action, ProductionId, StateId},
    parser_generator::{
        ParserGenerator,
        item::{LR0Item, LR1Item, LR1ItemSetBuilder},
    },
    symbol::TerminalId,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct LR0ItemSet {
    items: Vec<LR0Item>,
}

impl LR0ItemSet {
    fn new(items: HashSet<LR0Item>) -> Self {
        let mut items: Vec<LR0Item> = items.into_iter().collect();
        items[..].sort();
        Self { items }
    }

    fn contains(&self, item: &LR0Item) -> bool {
        self.items[..].binary_search(item).is_ok()
    }

    fn iter(&self) -> impl Iterator<Item = &LR0Item> {
        self.items.iter()
    }
}

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

pub(crate) fn get_name_or_default(
    s: &SymbolId,
    symbol_names: &HashMap<SymbolId, Option<String>>,
) -> String {
    symbol_names
        .get(s)
        .unwrap()
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or(s.id_string())
}

fn get_production_string(
    production: &(SymbolId, Vec<SymbolId>),
    symbol_names: &HashMap<SymbolId, Option<String>>,
) -> (String, String) {
    let production_rhs: Vec<String> = production
        .1
        .iter()
        .map(|s| get_name_or_default(s, symbol_names))
        .collect();

    let production_rhs = production_rhs.join(" ");

    let production_lhs_symbol = production.0;
    let production_lhs_name = get_name_or_default(&production_lhs_symbol, symbol_names);

    (production_lhs_name, production_rhs)
}

type PropagationTable = HashMap<(StateId, LR0Item), HashSet<(StateId, LR0Item)>>;
type GotoTable = HashMap<(StateId, SymbolId), StateId>;

impl<TokenType: Eq + Hash + Copy + Debug, R> ParserGenerator<TokenType, R> {
    pub(crate) fn build_parsing_tables(&mut self) -> Result<(), Vec<Conflict>> {
        let (initial_state, initial_production_core, lr0_states, goto_table) =
            self.build_lr0_states();

        let lr0_states_kernels = lr0_states
            .into_iter()
            .map(|(id, s)| (id, self.kernel(s)))
            .collect();

        let (mut spontaneous_propagations, propagation_table) =
            self.build_follow_propagations(lr0_states_kernels, &goto_table);

        spontaneous_propagations
            .get_mut(&initial_state)
            .unwrap()
            .insert_to_core_if_exists(
                &initial_production_core,
                &vec![self.eof].into_iter().collect(),
            );

        // self.dbg_states_builders(&spontaneous_propagations);
        // self.dbg_propagation_table(&propagation_table);

        let lr1_kernels = self.propagate_follows(spontaneous_propagations, propagation_table);

        // self.dbg_states_builders(&lr1_kernels);

        let lr1_states: HashMap<StateId, HashSet<LR1Item>> = lr1_kernels
            .into_iter()
            .map(|(state_id, kernel)| (state_id, self.lr1_closure(kernel.to_hash_set())))
            .collect();

        // self.dbg_states(&lr1_states);

        let action_table = self.build_action_table(
            initial_state,
            initial_production_core,
            &lr1_states,
            &goto_table,
        )?;

        // self.dbg_action_table(&action_table);

        self.action_table = action_table;
        self.goto = goto_table;

        Ok(())
    }

    fn build_lr0_states(&self) -> (StateId, LR0Item, HashMap<StateId, LR0ItemSet>, GotoTable) {
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
                    let new_set = LR0ItemSet::new(new_set);

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

        (
            first_set_id,
            first_item,
            Self::swap_key_values(id_map),
            goto_table,
        )
    }

    fn build_follow_propagations(
        &self,
        lr0_states_kernels: HashMap<StateId, LR0ItemSet>,
        goto_table: &GotoTable,
    ) -> (HashMap<StateId, LR1ItemSetBuilder>, PropagationTable) {
        let mut spontaneous_propagations =
            Self::initialize_spontaneous_propagation_map(&lr0_states_kernels);

        let mut propagation_table = PropagationTable::new();

        // for each (id, set) in lr0 states
        for (i, set) in lr0_states_kernels {
            // for each kernel item k
            for k in self.non_final_items(&set) {
                // c = closure([k, #])
                let numeral_item =
                    LR1Item::build(*k, vec![self.extra_symbol].into_iter().collect()).unwrap();
                let closure = self.lr1_closure(vec![numeral_item].into_iter().collect());
                // self.dbg_lr1_item_set(&closure);
                // for each i = (core, follow) = [A -> p.Xs, follow] in c
                for LR1Item { core, follow } in closure {
                    // j_id = gototable[id, X]
                    if self.get_production_symbols(core.production_id).1.len() == 0 {
                        continue; // epsilon production
                    }
                    let x = self.symbol_right_of_dot(&core).unwrap();
                    let j = goto_table.get(&(i, x)).unwrap();
                    // core_j = core.increased_index()
                    let core = self.increased_dot_position(core).unwrap();

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

        // dbg!(&spontaneous_propagations);

        (spontaneous_propagations, propagation_table)
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

                for (i_core, follow) in i.items.iter() {
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

    fn build_action_table(
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
                match self.symbol_right_of_dot(&item.core) {
                    None => {
                        for f in item.follow.iter() {
                            let action_key = (*state_id, *f);
                            let action_to_add = Action::Reduce(item.core.production_id);
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

    fn build_first_lr0_item_set(&self) -> (LR0Item, LR0ItemSet) {
        let first_item = LR0Item::new(self.first_production_id, 0);

        let mut first_item_set = HashSet::new();
        first_item_set.insert(first_item);
        first_item_set = self.lr0_closure(first_item_set);

        (first_item, LR0ItemSet::new(first_item_set))
    }

    fn get_goto_kernels(&self, item_set: &LR0ItemSet) -> HashMap<SymbolId, HashSet<LR0Item>> {
        let mut goto_symbols = HashMap::new();

        for item in item_set.items.iter() {
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

    fn non_final_items<'a>(
        &self,
        lr0_item_set: &'a LR0ItemSet,
    ) -> impl Iterator<Item = &'a LR0Item> {
        lr0_item_set.iter().filter(|item| {
            let production = self.productions.get(&item.production_id).unwrap();
            item.dot_position < production.rhs.len()
        })
    }

    fn kernel(&self, lr0_item_set: LR0ItemSet) -> LR0ItemSet {
        LR0ItemSet::new(
            lr0_item_set
                .items
                .into_iter()
                .filter(|item| {
                    item.dot_position > 0 || self.first_production_id == item.production_id
                })
                .collect(),
        )
    }

    fn initialize_spontaneous_propagation_map(
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

    fn get_production_symbols(&self, production_id: ProductionId) -> (SymbolId, Vec<SymbolId>) {
        let prod = self.productions.get(&production_id).unwrap();

        (
            SymbolId::from(prod.lhs),
            prod.rhs.iter().map(|s| *s).collect(),
        )
    }

    fn dbg_states(&self, states: &HashMap<StateId, HashSet<LR1Item>>) {
        let mut states = states
            .iter()
            .collect::<Vec<(&StateId, &HashSet<LR1Item>)>>();
        states[..].sort_by_key(|(s, _)| s.0);

        for (s, items) in states.iter() {
            eprintln!("{}: {{\n{}\n}}", s.0, self.dbg_lr1_item_set_str(items, 4))
        }
    }

    fn dbg_states_builders(&self, states: &HashMap<StateId, LR1ItemSetBuilder>) {
        let indent_str = (0..4).map(|_| " ").collect::<Vec<&str>>().join("");

        let states = states
            .iter()
            .map(|(state_id, kernel)| {
                format!(
                    "{}: {{\n{}\n}}",
                    state_id.0,
                    kernel
                        .items
                        .iter()
                        .map(|(core, follow)| {
                            let core = self.dbg_lr0_item_str(core);
                            let follow = self.dbg_terminal_set_str(follow);

                            format!("{}{}, {}", indent_str, core, follow)
                        })
                        .collect::<Vec<String>>()
                        .join("\n"),
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        eprintln!("{}", states);
    }

    fn dbg_propagation_table(
        &self,
        table: &HashMap<(StateId, LR0Item), HashSet<(StateId, LR0Item)>>,
    ) {
        let indent_str = (0..4).map(|_| " ").collect::<Vec<&str>>().join("");

        let str = table
            .iter()
            .map(|((state_id, core), to)| {
                format!(
                    "from {} in I{} to {{\n{}\n}}",
                    self.dbg_lr0_item_str(core),
                    state_id.0,
                    to.iter()
                        .map(|(state_id, core)| {
                            format!(
                                "{indent_str}{} in I{}",
                                self.dbg_lr0_item_str(core),
                                state_id.0
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        eprintln!("{}", str)
    }

    fn dbg_action_table(&self, action_table: &HashMap<(StateId, TerminalId), Action>) {
        let str = action_table
            .iter()
            .map(|((s, t), a)| {
                format!(
                    "({}, {}) -> {}",
                    s.0,
                    get_name_or_default(&SymbolId::from(*t), &self.symbols),
                    self.dbg_action_str(&a)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        eprintln!("{}", str)
    }

    fn dbg_action_str(&self, action: &Action) -> String {
        match action {
            Action::Shift(state_id) => format!("Shift {}", state_id.0),
            Action::Reduce(production_id) => {
                let (lhs, rhs) = self.get_production_symbols(*production_id);
                let (lhs, rhs) = get_production_string(&(lhs, rhs), &self.symbols);
                format!("Reduce {lhs} -> {rhs}",)
            }
            Action::Accept => format!("Accept"),
        }
    }

    fn dbg_lr1_item_set(&self, item_set: &HashSet<LR1Item>) {
        eprintln!(
            "Item set: {{\n{}\n}}",
            self.dbg_lr1_item_set_str(item_set, 4)
        )
    }

    pub(crate) fn dbg_lr1_item_set_builder(&self, item_set: &LR1ItemSetBuilder) {
        eprintln!(
            "Item set builder: {{\n{}\n}}",
            self.dbg_lr1_item_set_builder_str(item_set, 4)
        )
    }

    fn dbg_lr1_item_set_builder_str(
        &self,
        item_set: &LR1ItemSetBuilder,
        indent_level: u32,
    ) -> String {
        self.dbg_lr1_item_set_str(&item_set.clone().to_hash_set(), indent_level)
    }

    fn dbg_lr1_item_set_str(&self, item_set: &HashSet<LR1Item>, indent_level: u32) -> String {
        let indent_str = (0..indent_level)
            .map(|_| " ")
            .collect::<Vec<&str>>()
            .join("");

        item_set
            .iter()
            .map(|i| indent_str.to_string() + &self.dbg_lr1_item_str(i))
            .collect::<Vec<String>>()
            .join(",\n")
    }

    fn dbg_lr1_item_str(&self, item: &LR1Item) -> String {
        let core = self.dbg_lr0_item_str(&item.core);
        let follow = self.dbg_terminal_set_str(&item.follow);

        format!("{}, {}", core, follow)
    }

    fn dbg_terminal_set_str(&self, set: &HashSet<TerminalId>) -> String {
        set.iter()
            .map(|s| get_name_or_default(&SymbolId::from(*s), &self.symbols))
            .collect::<Vec<String>>()
            .join("/")
    }

    fn dbg_lr0_item_str(&self, item: &LR0Item) -> String {
        let (lhs, rhs) = self.get_production_symbols(item.production_id);

        let lhs = get_name_or_default(&lhs, &self.symbols);
        let rhs = rhs
            .iter()
            .map(|s| get_name_or_default(s, &self.symbols))
            .collect::<Vec<String>>();

        let before_dot = &rhs[..item.dot_position].join(" ");
        let after_dot = &rhs[item.dot_position..].join(" ");

        format!("{} -> {}.{}", lhs, before_dot, after_dot)
    }
}
