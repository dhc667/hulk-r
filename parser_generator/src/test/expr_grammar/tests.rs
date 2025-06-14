use crate::ParseError;
use crate::test::{
    expr_grammar::{token_type::TokenType, grammar},
    helpers,
};

fn parse(input: &str) -> Result<(), ParseError<TokenType>> {
    let answ = helpers::parse(grammar::lexer_parser, input);
    answ
}

#[test]
fn simple_arithmetic() {
    let answ = parse("x + y * z");
    assert!(answ.is_ok());
}

#[test]
fn function_call() {
    let answ = parse("f(x, y)");
    assert!(answ.is_ok());
}

#[test]
fn nested_function_calls() {
    let answ = parse("f(g(x), y + z * w)");
    assert!(answ.is_ok());
}

#[test]
fn empty_function_call() {
    let answ = parse("f()");
    assert!(answ.is_ok());
}

#[test]
fn complex_expression() {
    let answ = parse("a + b * c + f(x, y) * g(z)");
    assert!(answ.is_ok());
}

#[test]
fn error_missing_paren() {
    let answ = parse("f(x, y");
    assert!(answ.is_err());
}

#[test]
fn error_missing_comma() {
    let answ = parse("f(x y)");
    assert!(answ.is_err());
}

#[test]
#[should_panic]
fn error_invalid_operator() {
    let answ = parse("x / y");
    assert!(answ.is_err());
} 
