use std::collections::{HashMap, HashSet};

use crate::{
    automata_utils::transitionable::NDTransitionable, regex_engine::regex_ast::symbol::Symbol,
};

/// # Description
/// A Non-deterministic Finite Automaton (NFA).
/// ## Fields:
/// - `q0`: The initial state of the NFA.
/// - `qf`: The final state of the NFA.
/// - `d`: A mapping of transitions, where each key is a tuple of (state, symbol) and the value is a set of next states.
pub struct NFA {
    pub q0: usize,
    pub qf: usize,
    pub d: HashMap<(usize, Symbol), HashSet<usize>>,
}

impl NFA {
    /// Constructs a new `NFA` with the given initial state, final state, and transition map.
    /// # Arguments
    /// - `q0`: The initial state of the NFA.
    /// - `qf`: The final state of the NFA.
    /// - `d`: A mapping of transitions, where each key is a tuple of (state, symbol) and the value is a set of next states.
    /// # Returns
    /// A new `NFA` instance.
    pub fn new(q0: usize, qf: usize, d: HashMap<(usize, Symbol), HashSet<usize>>) -> Self {
        NFA { q0, qf, d }
    }

    /// Simulates the NFA on a given input string.
    /// # Arguments
    /// - `input`: A vector of characters representing the input string to be processed by the NFA.
    /// # Returns
    /// A boolean indicating whether the NFA accepts the input string (i.e., whether it reaches the final state).
    pub fn simulate(&self, input: Vec<char>) -> bool {
        let mut s = self.e_closure(&HashSet::from([self.q0]));

        for &c in input.iter() {
            s = self.e_closure(&self.move_to(&s, &Symbol::from(c)));
        }
        s.contains(&self.qf)
    }
}

impl NDTransitionable for NFA {
    fn get_transitions(&self) -> &HashMap<(usize, Symbol), HashSet<usize>> {
        &self.d
    }
}

/// A function to print the transition table of an NFA. For debugging and visualization purposes.
/// # Arguments
/// - `nfa`: A reference to the `NFA` instance whose transition table is to be printed.
pub fn print_transition_table(nfa: &NFA) {
    println!("Transition Table:");
    let mut transitions: Vec<_> = nfa.d.iter().collect();
    transitions.sort_by_key(|(key, _)| (key.0, key.1.clone()));
    for (key, value) in transitions {
        println!(
            "State: {}, Symbol: {:?} -> States: {:?}",
            key.0, key.1, value
        );
    }
    println!("Start State: {}", nfa.q0);
    println!("Accept State: {}", nfa.qf);
    println!();
}
