use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::{
    RegexParser,
    lexer_generator::{
        automata::{super_dfa::SuperDFA, super_nfa::SuperNFA},
        lexer_result::LexerResult,
        rule::Rule,
    },
    regex_engine::automata::{nfa::NFA, nfa_builder::NFABuilder},
};

/// # Description
/// This module defines a `Lexer` struct that represents a lexer for tokenizing input strings based on defined rules.
/// It uses a deterministic finite automata (DFA) as engine to efficiently tokenize input strings.
pub struct Lexer<TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq + Debug,
{
    rules: HashMap<TokenKind, Rule<TokenKind>>,
    engine: SuperDFA<TokenKind>,
}

impl<TokenKind> Lexer<TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq + Debug,
{
    /// Creates a new `Lexer` instance from a vector of `Rule`s.
    /// # Arguments
    /// * `rules`: A vector of `Rule` instances that define the tokenization rules.
    /// # Returns
    /// A new `Lexer` instance that can tokenize input strings based on the provided rules.
    pub fn new(rules: Vec<Rule<TokenKind>>) -> Self {
        let parser = RegexParser::new();
        let attributed_nfas: Vec<(NFA, TokenKind)> = rules
            .iter()
            .map(|rule| {
                let regex = parser.parse(&rule.pattern).unwrap();
                let mut builder = NFABuilder::new();
                let nfa = builder.build_from_regex(&regex);
                (nfa, rule.token_kind.clone())
            })
            .collect();
        let nfa = SuperNFA::new(&attributed_nfas);
        let engine = SuperDFA::new(&nfa);

        let rules = rules
            .into_iter()
            .map(|rule| (rule.token_kind.clone(), rule))
            .collect::<HashMap<_, _>>();
        Lexer { rules, engine }
    }

    /// Splits the input string into tokens based on the defined rules.
    /// # Arguments
    /// * `input`: The input string to be tokenized.
    /// # Returns
    /// A `LexerResult` containing the tokens recognized in the input string and the errors encountered.
    pub fn split<'a>(&self, input: &'a str) -> LexerResult<'a, TokenKind> {
        let mut result = self.engine.scan(input);

        result.tokens = result
            .tokens
            .into_iter()
            .filter(|token| self.rules.get(&token.ty).map_or(false, |rule| !rule.skip))
            .collect();
        result
    }
}
