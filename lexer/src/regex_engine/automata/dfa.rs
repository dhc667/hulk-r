use std::collections::{HashMap, HashSet};

pub struct DFA {
    /// Start state
    pub q0: usize,
    /// Accept states
    pub qf: HashSet<usize>,
    /// Transition function: (state, symbol) -> state
    pub d: HashMap<(usize, char), usize>,
}

impl DFA {
    pub fn new(q0: usize, qf: HashSet<usize>) -> Self {
        DFA {
            q0,
            qf,
            d: HashMap::new(),
        }
    }

    pub fn simulate(&self, input: Vec<char>) -> bool {
        let mut current_state = self.q0;

        for c in input {
            if let Some(&next_state) = self.d.get(&(current_state, c)) {
                current_state = next_state;
            } else {
                return false; // No valid transition
            }
        }

        self.qf.contains(&current_state) // Check if the final state is an accept state
    }
}
