use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{
    automata_utils::{
        marked_queue::MarkedQueue,
        representation::{to_set, to_str},
        transitionable::NDTransitionable,
    },
    lexer_generator::{automata::super_nfa::SuperNFA, lexer_chunk::LexerChunk},
};

pub struct SuperDFA<TokenKind>
where
    TokenKind: Clone + PartialEq,
{
    pub q0: usize,
    pub qf: HashMap<usize, TokenKind>,
    pub d: HashMap<(usize, char), usize>,
}

impl<TokenKind> SuperDFA<TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
{
    pub fn new(nfa: &SuperNFA<TokenKind>) -> Self {
        SuperDFA::from(nfa)
    }

    pub fn scan<'a>(&self, input: &'a str) -> Result<Vec<LexerChunk<'a, TokenKind>>, Vec<String>> {
        let mut chunks = Vec::new();
        let mut errors = Vec::new();
        let mut panic_mode = false;

        let mut i = 0;
        let chars = input.chars().collect::<Vec<_>>();

        let mut line = (0, 0); // (line, line start)

        while i < input.len() {
            let mut current_state = self.q0;
            let mut last_accepted: Option<(usize, TokenKind)> = None;
            let mut j = i;
            while j < input.len() {
                if self.qf.contains_key(&current_state) {
                    last_accepted = Some((j, self.qf[&current_state].clone()));
                }
                let c = chars[j];
                if c == '\n' {
                    line.0 += 1;
                    line.1 = j + 1;
                }
                let next_state = self.d.get(&(current_state, c));
                match next_state {
                    Some(&next) => {
                        current_state = next;
                        j += 1;
                    }
                    None => {
                        match last_accepted {
                            Some((index, ty)) => {
                                let chunk = LexerChunk::new(
                                    ty,
                                    &input[i..index + 1],
                                    line.0,
                                    i,
                                    index + 1,
                                );
                                chunks.push(chunk);
                                i = index + 1;
                                break;
                            }
                            None => {
                                // No valid transition found, enter panic mode
                                panic_mode = true;
                                errors.push(format!(
                                    "Lexical Error: Unexpected character '{}' at line: {}, column: {}",
                                    c, line.0, j - line.1 + 1 
                                ));
                                i += 1; // Skip the current character
                                break;
                            }
                        }
                    }
                }
            }
        }
        if panic_mode {
            return Err(errors);
        }
        Ok(chunks)
    }
}

impl<TokenKind> From<&SuperNFA<TokenKind>> for SuperDFA<TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
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

        let qf: HashMap<usize, TokenKind> = queue
            .iter()
            .filter_map(|s| {
                let set = to_set(s);
                set.iter()
                    .filter_map(|q| nfa.qf.get(q).cloned())
                    .max_by(|a, b| a.1.cmp(&b.1))
                    .map(|(kind, _)| (queue[s], kind))
            })
            .collect();

        SuperDFA { q0, qf, d }
    }
}
