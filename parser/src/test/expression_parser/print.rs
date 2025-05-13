use crate::grammar;
use ast::Expression;

#[test]
fn parses_print_expression() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse("print(1 + 2 + 3 * 4)").unwrap();
    assert_eq!(
        answ.as_function_call().unwrap().arguments[0]
            .as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .rhs
            .as_number_literal()
            .unwrap()
            .value,
        4.0
    );
}

#[test]
fn parses_print_exp_in_then_branch() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse("if (x + 3) print(h) else d");
    if let Ok(Expression::IfElse(if_else_exp)) = answ {
        let then_branch = &if_else_exp.then_expression;
        let else_branch = &if_else_exp.else_expression;

        assert_eq!(
            then_branch.as_function_call().unwrap().arguments[0]
                .as_variable()
                .unwrap()
                .id,
            "h"
        );
        assert_eq!(else_branch.as_variable().unwrap().id, "d");
    } else {
        panic!("Expected IfElseExpression");
    }
}

#[test]
fn parses_print_exp_in_condition() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse("if (print(x + 3)) print(h) else d");
    if let Ok(Expression::IfElse(if_else_exp)) = answ {
        let condition = &if_else_exp.condition;
        let then_branch = &if_else_exp.then_expression;
        let else_branch = &if_else_exp.else_expression;

        assert_eq!(
            condition.as_function_call().unwrap().arguments[0]
                .as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "x"
        );
        assert_eq!(
            then_branch.as_function_call().unwrap().arguments[0]
                .as_variable()
                .unwrap()
                .id,
            "h"
        );
        assert_eq!(else_branch.as_variable().unwrap().id, "d");
    } else {
        panic!("Expected IfElseExpression");
    }
}
