use lexer::lexer_generator::{lexer::Lexer, rule::Rule};

use crate::{test::calculator_grammar::token_type::TokenType, Parser, Token};

pub fn lexer_parser() -> (Lexer<TokenType>, Parser<TokenType, Option<i32>>) {
    let (lexer, parser) = grammar! {
        token_type: TokenType,
        return_type: Option<i32>,
        lexer_type: Lexer,
        rule_type: Rule,
        first_symbol: E,
        default_token_action: |tok: &Token<TokenType>| match tok.slice.parse::<i32>() {
            Ok(i) => Some(i),
            Err(_) => None,
        },

        productions: {
            E -> E Plus T = |v| Some(v[0].unwrap() + v[2].unwrap());
            E -> E Minus T = |v| Some(v[0].unwrap() - v[2].unwrap());
            E -> T = |v| Some(v[0].unwrap());

            T -> T Times F = |v| Some(v[0].unwrap() * v[2].unwrap());
            T -> T Div F = |v| Some(v[0].unwrap() / v[2].unwrap());
            T -> F = |v| Some(v[0].unwrap());

            F -> Number = |v| v[0];
            F -> Lpar E Rpar = |v| v[1];
        }

        terminals: {
            (Number, r"[1-9][0-9]*"),
            (Plus, r"\+"),
            (Minus, r"\-"),
            (Times, r"\*"),
            (Div, r"/"),
            (Lpar, r"\("),
            (Rpar, r"\)")
        }

        SKIP __Whitespace__ r"\s+";
    };

    (lexer, parser)
}
