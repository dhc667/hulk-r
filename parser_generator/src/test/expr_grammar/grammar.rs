use crate::test::{LexerDefiner, LexerWrapper};
use crate::{Parser, Token, grammar};

use crate::test::expr_grammar::token_type::TokenType;

pub fn lexer_parser() -> (LexerWrapper<TokenType>, Parser<TokenType, ()>) {
    let (lexer, parser) = grammar! {
        token_type: TokenType,
        return_type: (),
        lexer_definer_type: LexerDefiner,
        first_symbol: E,
        default_token_action: |tok: &Token<TokenType>| {
            eprintln!("Parsed token {:?}", tok.ty);
        },

        productions: {
            E -> E Plus T = |_| eprintln!("Parsed E -> E + T");
            E -> T = |_| eprintln!("Parsed E -> T");
            T -> T Aster F = |_| eprintln!("Parsed T -> T * F");
            T -> F = |_| eprintln!("Parsed T -> F");

            F -> Identifier OptionalBraceOptions Arguments
                = |_| eprintln!("Parsed F -> Identifier OptionalBraceOptions Arguments");

            OptionalBraceOptions -> LBrace Identifier RBrace
                = |_| eprintln!("Parsed OptionalBraceOptions -> {{Identifier}}");

            OptionalBraceOptions -> #Epsilon = |_| eprintln!("Parsed OptionalBraceOptions -> ε");
            Arguments -> #Epsilon = |_| eprintln!("Parsed Arguments -> ε");
            Arguments -> LParen ArgumentList RParen = |_| eprintln!("Parsed Arguments -> ( ArgumentList )");
            Arguments -> LParen RParen = |_| eprintln!("Parsed Arguments -> ( )");
            ArgumentList -> ArgumentList Comma E = |_| eprintln!("Parsed ArgumentList -> ArgumentList , E");
            ArgumentList -> E = |_| eprintln!("Parsed ArgumentList -> E");
        }

        terminals: {
            (Plus, r"\+"),
            (Aster, r"\*"),
            (Identifier, r"[A-Za-z][A-Za-z0-9]*"),
            (LParen, r"\("),
            (RParen, r"\)"),
            (LBrace, r"{"),
            (RBrace, r"}"),
            (Comma, r",")
        }

        skip: {
            (__Whitespace__, r"\s+")
        }

    };

    (lexer, parser)
}
