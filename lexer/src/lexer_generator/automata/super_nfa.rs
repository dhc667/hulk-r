use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use crate::{
    automata_utils::transitionable::NDTransitionable,
    regex_engine::{automata::nfa::NFA, regex_ast::symbol::Symbol},
};

pub struct SuperNFA<TokenKind>
where
    TokenKind: Clone + PartialEq,
{
    pub q0: usize,
    pub qf: HashMap<usize, TokenKind>,
    pub d: HashMap<(usize, Symbol), HashSet<usize>>,
}

impl<TokenKind> SuperNFA<TokenKind>
where
    TokenKind: Clone + PartialEq,
{
    pub fn new(attributed_nfas: &Vec<(NFA, TokenKind)>) -> Self {
        let q0 = 0; // Initial state for the SuperNFA
        let mut max_state = q0;

        let mut qf = HashMap::new();
        let mut d = HashMap::new();
        d.insert((q0, Symbol::Epsilon), HashSet::new());

        for (nfa, kind) in attributed_nfas {
            let offset = max_state + 1; // Offset for the current NFA's states

            // Add epsilon transitions from the SuperNFA's initial state to the NFA's initial state
            d.get_mut(&(q0, Symbol::Epsilon))
                .unwrap()
                .insert(nfa.q0 + offset);

            // Add transitions for the NFA
            for ((q, c), next) in &nfa.d {
                max_state = max(max_state, q + offset);
                max_state = max(max_state, next.iter().copied().max().unwrap_or(0));

                let next: HashSet<usize> = next.iter().map(|&s| s + offset).collect();
                d.entry((q + offset, c.clone()))
                    .or_insert_with(HashSet::new)
                    .extend(next);
            }
            // Add final states and their associated token kind
            qf.insert(nfa.qf, kind.clone());
        }

        SuperNFA { q0, qf, d }
    }
}

impl<TokenKind> NDTransitionable for SuperNFA<TokenKind>
where
    TokenKind: Clone + PartialEq,
{
    fn get_transitions(&self) -> &HashMap<(usize, Symbol), HashSet<usize>> {
        &self.d
    }
}
