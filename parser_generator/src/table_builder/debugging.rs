#![cfg(test)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::{
    StateId, SymbolId, TerminalId,
    debugging_helpers::{get_name_or_default, get_production_string},
    parser::Action,
    table_builder::{
        TableBuilder,
        items::{LR0Item, LR1Item, LR1ItemSetBuilder},
    },
};

impl<'b> TableBuilder<'b> {
    pub fn dbg_states(&self, states: &HashMap<StateId, HashSet<LR1Item>>) {
        let mut states = states
            .iter()
            .collect::<Vec<(&StateId, &HashSet<LR1Item>)>>();
        states[..].sort_by_key(|(s, _)| s.0);

        for (s, items) in states.iter() {
            let items = Self::lr1_item_set_to_core_follow_map(items);
            eprintln!("{}: {{\n{}\n}}", s.0, self.dbg_lr1_item_set_str(&items, 4))
        }
    }

    fn lr1_item_set_to_core_follow_map(
        item_set: &HashSet<LR1Item>,
    ) -> HashMap<LR0Item, HashSet<TerminalId>> {
        item_set
            .iter()
            .map(|it| (*it.core(), it.follow().clone()))
            .collect()
    }

    pub fn dbg_states_builders(&self, states: &HashMap<StateId, LR1ItemSetBuilder>) {
        let indent_str = (0..4).map(|_| " ").collect::<Vec<&str>>().join("");

        let states = states
            .iter()
            .map(|(state_id, kernel)| {
                format!(
                    "{}: {{\n{}\n}}",
                    state_id.0,
                    kernel
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

    pub fn dbg_propagation_table(
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

    pub fn dbg_lr1_item_set(&self, item_set: &HashSet<LR1Item>) {
        let item_set = Self::lr1_item_set_to_core_follow_map(item_set);

        eprintln!(
            "Item set: {{\n{}\n}}",
            self.dbg_lr1_item_set_str(&item_set, 4)
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
        self.dbg_lr1_item_set_str(
            &item_set
                .iter()
                .map(|(item, follow)| (*item, follow.clone()))
                .collect(),
            indent_level,
        )
    }

    fn dbg_lr1_item_set_str(
        &self,
        item_set: &HashMap<LR0Item, HashSet<TerminalId>>,
        indent_level: u32,
    ) -> String {
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

    pub fn dbg_lr1_item(&self, item: &LR1Item) {
        eprintln!("{}", self.dbg_lr1_item_str((item.core(), item.follow())))
    }

    fn dbg_lr1_item_str(&self, (core, follow): (&LR0Item, &HashSet<TerminalId>)) -> String {
        let core = self.dbg_lr0_item_str(core);
        let follow = self.dbg_terminal_set_str(follow);

        format!("{}, {}", core, follow)
    }

    fn dbg_terminal_set_str(&self, set: &HashSet<TerminalId>) -> String {
        set.iter()
            .map(|s| get_name_or_default(&SymbolId::from(*s), &self.symbols))
            .collect::<Vec<String>>()
            .join("/")
    }

    fn dbg_lr0_item_str(&self, item: &LR0Item) -> String {
        let (lhs, rhs) = self.get_production_symbols(*item.production_id());

        let lhs = get_name_or_default(&lhs, &self.symbols);
        let rhs = rhs
            .iter()
            .map(|s| get_name_or_default(s, &self.symbols))
            .collect::<Vec<String>>();

        let before_dot = &rhs[..item.dot_position()].join(" ");
        let after_dot = &rhs[item.dot_position()..].join(" ");

        format!("{} -> {}.{}", lhs, before_dot, after_dot)
    }
}
