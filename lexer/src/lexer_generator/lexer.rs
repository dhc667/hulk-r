use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::{
    RegexParser,
    lexer_generator::{
        automata::{super_dfa::SuperDFA, super_nfa::SuperNFA},
        lexer_chunk::LexerChunk,
        rule::Rule,
    },
    regex_engine::automata::{nfa::NFA, nfa_builder::NFABuilder},
};

pub struct Lexer<TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq + Debug,
{
    pub rules: HashMap<TokenKind, Rule<TokenKind>>,
    pub engine: SuperDFA<TokenKind>,
}

impl<TokenKind> Lexer<TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq + Debug,
{
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

    pub fn split<'a>(&self, input: &'a str) -> Result<Vec<LexerChunk<'a, TokenKind>>, Vec<String>> {
        let result = self.engine.scan(input);
        let Ok(tokens) = result else {
            return Err(result.err().unwrap());
        };

        Ok(tokens
            .into_iter()
            .filter(|token| self.rules.get(&token.ty).map_or(false, |rule| !rule.skip))
            .collect())
    }
}
