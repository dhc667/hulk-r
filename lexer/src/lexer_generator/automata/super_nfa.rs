use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{
    automata_utils::transitionable::NDTransitionable,
    regex_engine::{automata::nfa::NFA, regex_ast::symbol::Symbol},
};

/// # Description
/// This module defines a `SuperNFA` struct that represents a Non-deterministic Finite Automaton (NFA)
/// constructed from multiple attributed NFAs.
/// ## Fields:
/// - `q0`: The initial state of the SuperNFA.
/// - `qf`: A mapping of final states to their associated token kind and priority.
/// - `d`: A mapping of transitions, where each key is a tuple of (state, symbol) and the value is a set of next states.
/// ## Methods:
/// - `new`: Constructs a new `SuperNFA` from a vector of attributed NFAs, where each NFA is associated with a token kind.
pub struct SuperNFA<TokenKind>
where
    TokenKind: Clone + PartialEq,
{
    pub q0: usize,
    /// Final states and their associated token kind
    /// Each key is a final state index, and the value is the priority of the associated token kind.
    pub qf: HashMap<usize, (TokenKind, usize)>,
    pub d: HashMap<(usize, Symbol), HashSet<usize>>,
}

impl<TokenKind> SuperNFA<TokenKind>
where
    TokenKind: Clone + PartialEq + Debug,
{
    /// Constructs a new `SuperNFA` from a vector of attributed NFAs.
    /// # Arguments
    /// * `attributed_nfas`: A vector of tuples, where each tuple contains an NFA and its associated token kind.
    /// # Returns
    /// A new `SuperNFA` instance that represents the union of the provided NFAs.
    /// # Notes
    /// The initial state of the SuperNFA is always `0`, and the states of each NFA are offset by their index
    /// in the input vector to ensure unique state indices.
    /// The final states of the SuperNFA are mapped to their associated token kind and priority,
    /// where the priority is determined by the index of the NFA in the input vector.
    /// The transitions of the SuperNFA are constructed by adding epsilon transitions from the initial state to each NFA's initial state,
    pub fn new(attributed_nfas: &Vec<(NFA, TokenKind)>) -> Self {
        let q0 = 0; // Initial state for the SuperNFA
        let mut max_state = q0;

        let mut qf = HashMap::new();
        let mut d = HashMap::new();
        d.insert((q0, Symbol::Epsilon), HashSet::new());

        for (i, (nfa, kind)) in attributed_nfas.iter().enumerate() {
            let offset = max_state + 1; // Offset for the current NFA's states

            // Add epsilon transitions from the SuperNFA's initial state to the NFA's initial state
            d.get_mut(&(q0, Symbol::Epsilon))
                .unwrap()
                .insert(nfa.q0 + offset);

            // Add transitions for the NFA
            for ((q, c), next) in &nfa.d {
                let next: HashSet<usize> = next.iter().map(|&s| s + offset).collect();

                max_state = max(max_state, q + offset);
                max_state = max(max_state, next.iter().cloned().max().unwrap_or(0));

                d.entry((q + offset, c.clone()))
                    .or_insert_with(HashSet::new)
                    .extend(next);
            }
            // Add final states and their associated token kind
            qf.insert(nfa.qf + offset, (kind.clone(), i));
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
