use crate::Lex;

use crate::test::lrvalue_grammar::grammar;

#[test]
fn simple_parse() {
    let (lexer, parser) = grammar::lexer_parser();

    let tokens = lexer.split("A = B").unwrap();

    parser.parse(tokens).unwrap();
}

#[test]
fn complex_parse() {
    let (lexer, parser) = grammar::lexer_parser();

    let tokens = lexer.split("**A = B").unwrap();

    parser.parse(tokens).unwrap();
}
