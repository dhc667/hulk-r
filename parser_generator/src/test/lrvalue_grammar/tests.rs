use crate::Token;

use crate::test::lrvalue_grammar::grammar;

#[test]
fn simple_parse() {
    let (lexer, parser) = grammar::lexer_parser();

    let tokens = lexer
        .split("A = B")
        .unwrap()
        .iter()
        .map(|chunk| Token::new(chunk.ty, chunk.slice.to_string(), chunk.start, chunk.end))
        .collect();

    parser.parse(tokens).unwrap();
}

#[test]
fn complex_parse() {
    let (lexer, parser) = grammar::lexer_parser();

    let tokens = lexer
        .split("**A = B")
        .unwrap()
        .iter()
        .map(|chunk| Token::new(chunk.ty, chunk.slice.to_string(), chunk.start, chunk.end))
        .collect();

    parser.parse(tokens).unwrap();
}
