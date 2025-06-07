use std::collections::{HashMap, HashSet};

use crate::regex_engine::regex_ast::symbol::Symbol;

pub struct NFA {
    pub q0: usize,
    pub qf: usize,
    pub d: HashMap<(usize, Symbol), HashSet<usize>>,
}

impl NFA {
    pub fn new(q0: usize, qf: usize, d: HashMap<(usize, Symbol), HashSet<usize>>) -> Self {
        NFA { q0, qf, d }
    }

    pub fn e_closure(&self, t_set: &HashSet<usize>) -> HashSet<usize> {
        let mut closure = t_set.clone();
        let mut stack: Vec<usize> = t_set.iter().cloned().collect();

        while let Some(state) = stack.pop() {
            if let Some(next_states) = self.d.get(&(state, Symbol::Epsilon)) {
                for &next_state in next_states {
                    if closure.insert(next_state) {
                        stack.push(next_state);
                    }
                }
            }
        }

        closure
    }

    pub fn move_to(&self, t_set: &HashSet<usize>, symbol: &Symbol) -> HashSet<usize> {
        let mut next_states = HashSet::new();

        for &state in t_set {
            if let Some(states) = self.d.get(&(state, symbol.clone())) {
                for &next_state in states {
                    next_states.insert(next_state);
                }
            }
        }

        next_states
    }
}
