use std::collections::{HashMap, HashSet};

use crate::{
    automata_utils::{
        marked_queue::MarkedQueue,
        representation::{to_set, to_str},
        transitionable::NDTransitionable,
    },
    regex_engine::automata::nfa::NFA,
};

/// # Description
/// This module defines a `DFA` struct that represents a Deterministic Finite Automaton (DFA).
/// ## Fields:
/// - `q0`: The initial state of the DFA.
/// - `qf`: A set of accept states.
/// - `d`: A mapping of transitions, where each key is a tuple of (state, character) and the value is the next state.
/// ## Methods:
/// - `new`: Constructs a new `DFA` with a given start state and accept states.
/// - `simulate`: Simulates the DFA on a given input string and returns whether it accepts the input.
pub struct DFA {
    /// Start state
    pub q0: usize,
    /// Accept states
    pub qf: HashSet<usize>,
    /// Transition function: (state, symbol) -> state
    pub d: HashMap<(usize, char), usize>,
}

impl DFA {
    /// Constructs a new `DFA` with a given start state and accept states.
    /// # Arguments
    /// - `q0`: The initial state of the DFA.
    /// - `qf`: A set of accept states.
    pub fn new(q0: usize, qf: HashSet<usize>) -> Self {
        DFA {
            q0,
            qf,
            d: HashMap::new(),
        }
    }

    /// Simulates the DFA on a given input string and returns whether it accepts the input.
    /// # Arguments
    /// - `input`: A vector of characters representing the input string to be processed by the DFA.
    /// # Returns
    /// A boolean indicating whether the DFA accepts the input string.
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

impl From<NFA> for DFA {
    /// Constructs a `DFA` from a given `NFA`.
    /// # Arguments
    /// - `nfa`: An instance of `NFA` from which the DFA will be constructed.
    /// # Returns
    /// A new `DFA` instance that represents the equivalent DFA of the provided NFA.
    fn from(nfa: NFA) -> Self {
        let q0: usize = 0;
        let mut d: HashMap<(usize, char), usize> = HashMap::new();

        let mut queue = MarkedQueue::new();

        let e0_set = nfa.e_closure(&HashSet::from([nfa.q0]));
        let e0 = to_str(&e0_set);
        queue.add_unmarked(e0.clone());
        while let Some(t) = queue.pop_unmarked() {
            let t_set = to_set(&t);
            for c in 0..=255u8 {
                let a = char::from(c);
                let u_set = nfa.e_closure(&nfa.move_to(&t_set, &a.into()));
                if u_set.is_empty() {
                    continue;
                }
                let u = to_str(&u_set);
                if !queue.contains(&u) {
                    queue.add_unmarked(u.clone());
                }
                d.insert((queue[&t], a), queue[&u]);
            }
        }

        let qf: HashSet<usize> = queue
            .iter()
            .map(|s| to_set(s))
            .filter(|s| s.contains(&nfa.qf))
            .map(|s| queue[&to_str(&s)])
            .collect();

        DFA { q0, qf, d }
    }
}
