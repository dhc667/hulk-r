use lexer::lexer_generator::lexer::Lexer;
use lexer::lexer_generator::rule::Rule;

use crate::{Parser, Token, grammar};

use crate::test::lrvalue_grammar::token_type::TokenType;


pub fn lexer_parser() -> (Lexer<TokenType>, Parser<TokenType, ()>) {
    let (lexer, parser) = grammar! {
        token_type: TokenType,
        return_type: (),
        lexer_type: Lexer,
        rule_type: Rule,
        first_symbol: S,
        default_token_action: |tok: &Token<TokenType>| {
            eprintln!("Parsed token {:?}", tok.ty);
        },

        productions: {
            S -> L Equal R = |_| eprintln!("Parsed S -> L = R");
            S -> R = |_| eprintln!("Parsed S -> R");
            L -> Aster R = |_| eprintln!("Parsed L -> *R");
            L -> Identifier = |_| eprintln!("Parsed L -> id");
            R -> L = |_| eprintln!("Parsed L -> R");
        }

        terminals: {
            (Equal, "="),
            (Identifier, r"[A-Za-z][A-Za-z0-9]*"),
            (Aster, r"\*")
        }

        SKIP __Whitespace__ r"\s+";
    };

    (lexer, parser)
}
