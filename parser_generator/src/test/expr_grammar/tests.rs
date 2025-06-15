use crate::ParseError;
use crate::test::{
    expr_grammar::{grammar, token_type::TokenType},
    helpers,
};

fn parse(input: &str) -> Result<(), ParseError<TokenType>> {
    let answ = helpers::parse(grammar::lexer_parser, input);
    answ
}

#[test]
fn simple_arithmetic() {
    let answ = parse("x + y * z");
    answ.unwrap();
}

#[test]
fn function_call() {
    let answ = parse("f(x, y)");
    answ.unwrap();
}

#[test]
fn nested_function_calls() {
    let answ = parse("f(g(x), y + z * w)");
    answ.unwrap();
}

#[test]
fn empty_function_call() {
    let answ = parse("f()");
    answ.unwrap();
}

#[test]
fn complex_expression() {
    let answ = parse("a {hello} (a, b, c) + b {hello} * c + f(x, y) * g(z)");
    answ.unwrap();
}

#[test]
fn error_missing_paren() {
    let answ = parse("f(x, y");
    answ.unwrap_err();
}

#[test]
fn error_missing_comma() {
    let answ = parse("f(x y)");
    answ.unwrap_err();
}

#[test]
#[should_panic]
fn error_invalid_operator() {
    let answ = parse("x / y");
    answ.unwrap_err();
}

#[test]
fn error_arguments_before_options() {
    let answ = parse("x + y (a) {hello}");
    answ.unwrap_err();
}
