use std::collections::{HashMap, HashSet};

use crate::{
    automata_utils::{
        marked_queue::MarkedQueue,
        representation::{to_set, to_str},
        transitionable::NDTransitionable,
    },
    lexer_generator::automata::super_nfa::SuperNFA,
};

pub struct SuperDFA<TokenKind>
where
    TokenKind: Clone + PartialEq,
{
    pub q0: usize,
    pub qf: HashMap<usize, Vec<TokenKind>>,
    pub d: HashMap<(usize, char), usize>,
}

impl<TokenKind> SuperDFA<TokenKind>
where
    TokenKind: Clone + PartialEq,
{
    pub fn new(nfa: &SuperNFA<TokenKind>) -> Self {
        SuperDFA::from(nfa)
    }
}

impl<TokenKind> From<&SuperNFA<TokenKind>> for SuperDFA<TokenKind>
where
    TokenKind: Clone + PartialEq,
{
    fn from(nfa: &SuperNFA<TokenKind>) -> Self {
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

        let qf: HashMap<usize, Vec<TokenKind>> = queue
            .iter()
            .map(|s| to_set(s))
            .map(|s| {
                let mut finals = s
                    .iter()
                    .map(|s| nfa.qf.get(s).cloned())
                    .flatten()
                    .collect::<Vec<_>>();
                finals.sort_by(|a, b| a.1.cmp(&b.1));
                (
                    to_str(&s),
                    finals.iter().map(|f| f.0.clone()).collect::<Vec<_>>(),
                )
            })
            .map(|(s, finals)| (queue[&s], finals))
            .collect();

        SuperDFA { q0, qf, d }
    }
}
