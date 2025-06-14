
use lexer::lexer_generator::{lexer::Lexer, rule::Rule};
use crate::{grammar, Token};

use crate::test::non_lalr_grammar::token_type::TokenType;

#[test]
#[should_panic]
// Compilers Principles, Techniques and Tools second edition pp 267
pub fn lexer_parser() {
    grammar! {
        token_type: TokenType,
        return_type: (),
        lexer_type: Lexer,
        rule_type: Rule,
        first_symbol: S,
        default_token_action: |tok: &Token<TokenType>| {
            eprintln!("Parsed token {:?}", tok.ty);
        },

        productions: {
            S -> a A d = |_| {};
            S -> b B d = |_| {};
            S -> a B e = |_| {};
            S -> b A e = |_| {};
            A -> c = |_| {};
            B -> c = |_| {};
        }

        terminals: {
            (a, "a"),
            (b, "b"),
            (c, "c"),
            (d, "d"),
            (e, "e"),
        }
    };
}
