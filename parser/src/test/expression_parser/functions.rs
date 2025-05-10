use ast::VisitableExpression;

use crate::{grammar::ExpressionParser, visitors::echo_visitor::EchoVisitor};

#[test]
fn simple_function_call() {
    let p = ExpressionParser::new();

    let answ = p.parse("a(1, 2)").unwrap();

    assert_eq!(answ.as_function_call().unwrap().identifier.id, "a");
    assert_eq!(
        answ.as_function_call().unwrap().arguments[1]
            .as_number_literal()
            .unwrap()
            .value,
        2.0
    );
}

#[test]
fn nested_function_call() {
    let p = ExpressionParser::new();

    let answ = p.parse("a(a(1, \"hello\"), b)").unwrap();

    assert_eq!(
        answ.as_function_call().unwrap().arguments[0]
            .as_function_call()
            .unwrap()
            .arguments[1]
            .as_string_literal()
            .unwrap()
            .string,
        "hello"
    );

    assert_eq!(
        answ.as_function_call().unwrap().arguments[1]
            .as_variable()
            .unwrap()
            .id,
        "b"
    );
}

#[test]
fn operated_function_calls() {
    let p = ExpressionParser::new();

    let answ = p.parse("a() + b() * (f(x) - a * 4)/2").unwrap();

    let mut v = EchoVisitor::new();
    println!("{}", p.parse("a() + b() * (f(x) - a * 4)/2").unwrap().accept(&mut v));

    assert_eq!(
        answ.as_bin_op()
            .unwrap()
            .rhs // ( b() * (...) ) / 2
            .as_bin_op()
            .unwrap()
            .lhs // b() * (...)
            .as_bin_op()
            .unwrap()
            .lhs // b()
            .as_function_call()
            .unwrap()
            .identifier.id,
        "b"
    );


    assert_eq!(
        answ.as_bin_op()
            .unwrap()
            .rhs // ( b() * (...) ) / 2
            .as_bin_op()
            .unwrap()
            .lhs // b() * (...)
            .as_bin_op()
            .unwrap()
            .rhs // (f(x) - ...)
            .as_bin_op()
            .unwrap()
            .lhs // f(x)
            .as_function_call()
            .unwrap()
            .arguments[0]
            .as_variable()
            .unwrap()
            .id,
        "x"
    );
}
