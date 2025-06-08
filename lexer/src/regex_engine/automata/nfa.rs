use std::collections::{HashMap, HashSet};

use crate::{
    automata_utils::transitionable::NDTransitionable, regex_engine::regex_ast::symbol::Symbol,
};

pub struct NFA {
    pub q0: usize,
    pub qf: usize,
    pub d: HashMap<(usize, Symbol), HashSet<usize>>,
}

impl NFA {
    pub fn new(q0: usize, qf: usize, d: HashMap<(usize, Symbol), HashSet<usize>>) -> Self {
        NFA { q0, qf, d }
    }

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
