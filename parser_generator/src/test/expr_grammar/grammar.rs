use lexer::lexer_generator::lexer::Lexer;
use lexer::lexer_generator::rule::Rule;

use crate::{Parser, Token, grammar};

use crate::test::expr_grammar::token_type::TokenType;

pub fn lexer_parser() -> (Lexer<TokenType>, Parser<TokenType, ()>) {
    let (lexer, parser) = grammar! {
        token_type: TokenType,
        return_type: (),
        lexer_type: Lexer,
        rule_type: Rule,
        first_symbol: E,
        default_token_action: |tok: &Token<TokenType>| {
            eprintln!("Parsed token {:?}", tok.ty);
        },

        productions: {
            E -> E Plus T = |_| eprintln!("Parsed E -> E + T");
            E -> T = |_| eprintln!("Parsed E -> T");
            T -> T Aster F = |_| eprintln!("Parsed T -> T * F");
            T -> F = |_| eprintln!("Parsed T -> F");
            F -> Identifier Arguments = |_| eprintln!("Parsed F -> Identifier Arguments");
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
            (Comma, r",")
        }

        SKIP __Whitespace__ r"\s+";
    };

    (lexer, parser)
} 
