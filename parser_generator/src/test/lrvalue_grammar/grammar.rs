use crate::test::{LexerDefiner, LexerWrapper};
use crate::{Parser, Token, grammar};

use crate::test::lrvalue_grammar::token_type::TokenType;

pub fn lexer_parser() -> (LexerWrapper<TokenType>, Parser<TokenType, ()>) {
    let (lexer, parser) = grammar! {
        token_type: TokenType,
        return_type: (),
        lexer_definer_type: LexerDefiner,
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

        skip: {
            (__Whitespace__, r"\s+")
        }

    };

    (lexer, parser)
}
