use std::collections::{HashMap, HashSet};

use crate::regex_engine::regex_ast::symbol::Symbol;

pub trait NDTransitionable {
    fn get_transitions(&self) -> &HashMap<(usize, Symbol), HashSet<usize>>;

    fn e_closure(&self, t_set: &HashSet<usize>) -> HashSet<usize> {
        let mut closure = t_set.clone();
        let mut stack: Vec<usize> = t_set.iter().cloned().collect();

        while let Some(state) = stack.pop() {
            if let Some(next_states) = self.get_transitions().get(&(state, Symbol::Epsilon)) {
                for &next_state in next_states {
                    if closure.insert(next_state) {
                        stack.push(next_state);
                    }
                }
            }
        }

        closure
    }

    fn move_to(&self, t_set: &HashSet<usize>, symbol: &Symbol) -> HashSet<usize> {
        let mut next_states = HashSet::new();

        for &state in t_set {
            if let Some(states) = self.get_transitions().get(&(state, symbol.clone())) {
                for &next_state in states {
                    next_states.insert(next_state);
                }
            }
        }

        next_states
    }
}
