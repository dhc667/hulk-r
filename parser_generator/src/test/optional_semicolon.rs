use crate::parser::Parser;
use crate::test::LexerWrapper;
use crate::test::helpers::{LexerDefiner, parse};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenType {
    A,
    SEMICOLON,
    WHITESPACE,
}

#[derive(Debug, PartialEq)]
enum ReturnType {
    A,
    Empty,
}

fn lexer_parser() -> (LexerWrapper<TokenType>, Parser<TokenType, ReturnType>) {
    let (lexer, parser) = grammar! {
        token_type: TokenType,
        return_type: ReturnType,
        lexer_definer_type: LexerDefiner,
        first_symbol: S,
        default_token_action: |_: &_| ReturnType::Empty,

        productions: {
            S -> A OptionalSemicolon = { |_| ReturnType::A };
            OptionalSemicolon -> #Epsilon = { |_| ReturnType::Empty };
            OptionalSemicolon -> SEMICOLON = { |_| ReturnType::Empty };
        }

        terminals: {
            (A, r"a"),
            (SEMICOLON, r";"),
        }

        skip: {
            (WHITESPACE, r"\s+"),
        }
    };

    (lexer, parser)
}

#[test]
fn test_optional_semicolon() {
    // Test with just 'a'
    let input = "a";
    let result = parse(lexer_parser, input).unwrap();
    assert_eq!(result, ReturnType::A);

    // Test with 'a;'
    let input = "a;";
    let result = parse(lexer_parser, input).unwrap();
    assert_eq!(result, ReturnType::A);

    // Test with whitespace
    let input = "a ;";
    let result = parse(lexer_parser, input).unwrap();
    assert_eq!(result, ReturnType::A);
}
