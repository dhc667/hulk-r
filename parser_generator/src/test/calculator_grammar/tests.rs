use crate::ParseError;

use crate::test::{
    calculator_grammar::{grammar, token_type::TokenType},
    helpers,
};

fn parse(input: &str) -> Result<Option<i32>, ParseError<TokenType>> {
    let answ = helpers::parse(grammar::lexer_parser, input);

    answ
}

#[test]
fn sum_test() {
    let answ = parse("2 + 3 + 1 - 5").unwrap().unwrap();

    assert_eq!(answ, 1);
}

#[test]
fn multiplication_test() {
    let answ = parse("2 * 3 / 3").unwrap().unwrap();

    assert_eq!(answ, 2);
}

#[test]
fn composition_test() {
    let answ = parse("3 + 2 * 3").unwrap().unwrap();

    assert_eq!(answ, 9);
}

#[test]
fn parenthesis_test() {
    let answ = parse("(3 + 2) * 3").unwrap().unwrap();

    assert_eq!(answ, 15);
}

#[test]
fn complex_test() {
    let answ = parse("2 - 2 + 3*2 - 6 / 3 / 2 - 6 - 1 + (4 + 3) / 7").unwrap().unwrap();

    assert_eq!(answ, -1);
}
