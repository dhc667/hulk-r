use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use error_handler::error::lexical::invalid_character::InvalidCharacterError;

use crate::{
    automata_utils::{
        marked_queue::MarkedQueue,
        representation::{to_set, to_str},
        transitionable::NDTransitionable,
    },
    lexer_generator::{
        automata::super_nfa::SuperNFA, lexer_chunk::LexerChunk, lexer_result::LexerResult,
    },
};

/// # Description
/// This module defines a `SuperDFA` struct that represents a Deterministic Finite Automaton (DFA)
/// constructed from a Non-deterministic Finite Automaton (NFA).
/// ## Fields:
/// - `q0`: The initial state of the DFA.
/// - `qf`: A mapping of final states to their corresponding token kinds.
/// - `d`: A mapping of transitions, where each key is a tuple of (state, character) and the value is the next state.
/// ## Methods:
/// - `new`: Constructs a new `SuperDFA` from a given `SuperNFA`.
/// - `scan`: Scans an input string and returns a vector of `LexerChunk` instances representing recognized tokens, or a vector of error messages.
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
    TokenKind: Clone + PartialEq + Hash + Eq + Debug,
{
    /// Constructs a new `SuperDFA` from a given `SuperNFA`.
    /// # Arguments
    /// - `nfa`: A reference to a `SuperNFA` instance from which the DFA will be constructed.
    /// # Returns
    /// A new `SuperDFA` instance that represents the equivalent DFA of the provided NFA.
    pub fn new(nfa: &SuperNFA<TokenKind>) -> Self {
        SuperDFA::from(nfa)
    }

    /// Scans an input string and returns a vector of `LexerChunk` instances representing recognized tokens,
    /// or a vector of error messages if any lexical errors are encountered.
    /// # Arguments
    /// - `input`: A string slice that contains the input to be scanned.
    /// # Returns
    /// A LexerResult object containing errors and tokens recognized.
    pub fn scan<'a>(&self, input: &'a str) -> LexerResult<'a, TokenKind> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut pos = 0;
        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let mut line = (0, 0); // (line number, line start)

        while pos < len {
            let mut state = self.q0;
            let mut last_accepting: Option<(usize, &TokenKind)> = None;
            let mut current_line = line;
            let mut i = pos;
            while i < len {
                let c = chars[i];
                // Si es un salto de línea, actualiza la línea
                if c == '\n' {
                    current_line.0 += 1;
                    current_line.1 = i;
                }
                if let Some(&next_state) = self.d.get(&(state, c)) {
                    state = next_state;
                    if let Some(kind) = self.qf.get(&state) {
                        last_accepting = Some((i + 1, kind));
                    }
                    i += 1;
                } else {
                    break;
                }
            }
            if let Some((end, kind)) = last_accepting {
                let slice = &input[pos..end];
                tokens.push(LexerChunk {
                    ty: kind.clone(),
                    slice,
                    line: line.0,
                    start: pos,
                    end,
                });
                line = current_line;
                pos = end;
            } else {
                if pos == 0 || chars[pos - 1] != chars[pos] {
                    errors.push(InvalidCharacterError::new(chars[pos], pos).into());
                }
                line = current_line;
                pos += 1;
            }
        }
        LexerResult { tokens, errors }
    }
}

impl<TokenKind> From<&SuperNFA<TokenKind>> for SuperDFA<TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq + Debug,
{
    /// # Description
    /// Constructs a `SuperDFA` from a given `SuperNFA`.
    /// # Arguments
    /// - `nfa`: A reference to a `SuperNFA` instance from which the DFA will be constructed.
    /// # Returns
    /// A new `SuperDFA` instance that represents the equivalent DFA of the provided NFA.
    /// This method uses the subset construction algorithm to convert the NFA into a DFA.
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
                    .min_by(|a, b| a.1.cmp(&b.1))
                    .map(|(kind, _)| (queue[s], kind))
            })
            .collect();

        SuperDFA { q0, qf, d }
    }
}

/// # Description
/// This function prints the structure of a `SuperDFA` to the console. For debugging purposes.
pub fn print_dfa<TokenKind>(dfa: &SuperDFA<TokenKind>)
where
    TokenKind: Clone + PartialEq + Hash + Eq + std::fmt::Debug,
{
    println!("SuperDFA:");
    println!("  Initial state: {}", dfa.q0);
    println!("  Final states:");
    for (state, kind) in &dfa.qf {
        println!("    State {}: {:?}", state, kind);
    }
    println!("  Transitions:");
    let mut d: Vec<_> = dfa.d.iter().collect();
    d.sort();

    for ((state, symbol), next) in d {
        println!("    ({}, {:?}) -> {:?}", state, symbol, next);
    }
    println!();
}

/// # Description
/// This function prints the structure of a `SuperNFA` to the console. For debugging purposes.
pub fn print_nfa<TokenKind>(nfa: &SuperNFA<TokenKind>)
where
    TokenKind: Clone + PartialEq + std::fmt::Debug,
{
    println!("SuperNFA:");
    println!("  Initial state: {}", nfa.q0);
    println!("  Final states:");
    for (state, (kind, priority)) in &nfa.qf {
        println!("    State {}: {:?} (priority {})", state, kind, priority);
    }
    println!("  Transitions:");
    let mut d: Vec<_> = nfa.d.iter().collect();
    d.sort_by(|(a, _), (b, _)| a.cmp(b));

    for ((state, symbol), next_states) in d {
        println!(
            "    ({}, {:?}) -> {:?}",
            state,
            symbol,
            next_states.iter().collect::<Vec<_>>()
        );
    }
    println!();
}
