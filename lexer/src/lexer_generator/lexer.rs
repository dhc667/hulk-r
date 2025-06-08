use std::{collections::HashMap, hash::Hash};

use crate::{
    RegexParser,
    lexer_generator::{
        automata::{super_dfa::SuperDFA, super_nfa::SuperNFA},
        rule::Rule,
    },
    regex_engine::automata::{nfa::NFA, nfa_builder::NFABuilder},
};

pub struct Lexer<TokenKind, TokenType>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
{
    pub rules: HashMap<TokenKind, Rule<TokenKind, TokenType>>,
    pub engine: SuperDFA<TokenKind>,
}

impl<TokenKind, TokenType> Lexer<TokenKind, TokenType>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
{
    pub fn new(rules: Vec<Rule<TokenKind, TokenType>>) -> Self {
        let rules = rules
            .into_iter()
            .map(|rule| (rule.token_kind.clone(), rule))
            .collect::<HashMap<_, _>>();

        let parser = RegexParser::new();
        let attributed_nfas: Vec<(NFA, TokenKind)> = rules
            .iter()
            .map(|(_, rule)| {
                let regex = parser.parse(&rule.pattern).unwrap();
                let mut builder = NFABuilder::new();
                let nfa = builder.build_from_regex(&regex);
                (nfa, rule.token_kind.clone())
            })
            .collect();
        let nfa = SuperNFA::new(&attributed_nfas);
        let engine = SuperDFA::new(&nfa);

        Lexer { rules, engine }
    }

    pub fn split<'a>(&self, input: &'a str) -> Result<Vec<TokenType>, Vec<String>> {
        let result = self.engine.scan(input);
        let Ok(tokens) = result else {
            return Err(result.err().unwrap());
        };

        let mut result = Vec::new();
        for token in &tokens {
            if let Some(rule) = self.rules.get(&token.ty) {
                if let Some(action) = &rule.action {
                    result.push(action(
                        &token.ty,
                        token.slice,
                        token.line,
                        token.start,
                        token.end,
                    ));
                }
            }
        }
        Ok(result)
    }
}
