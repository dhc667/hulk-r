use crate::ParseError;

use crate::test::{
    expression_list::{TokenType, grammar, return_type::ReturnType},
    helpers,
};

fn parse(input: &str) -> Result<ReturnType, ParseError<TokenType>> {
    let answ = helpers::parse(grammar::lexer_parser, input);

    answ
}

#[test]
fn simple_test() {
    let answ = parse("1, 2 + 3, 3*3 + 1");

    let mut l = answ.unwrap().as_expression_list().unwrap();

    assert_eq!(l.remove(0), 1);
    assert_eq!(l.remove(0), 5);
    assert_eq!(l.remove(0), 10);
}

#[test]
fn error() {
    let answ = parse("1,,3");

    answ.unwrap_err();
}
