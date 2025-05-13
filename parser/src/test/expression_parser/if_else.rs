use crate::grammar;
use ast::Expression;

#[test]
fn parses_if_else_expression() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse("if (x - 5) y else z").unwrap();
    if let Expression::IfElse(if_else_exp) = answ {
        let condition = &if_else_exp.condition;
        let then_branch = &if_else_exp.then_expression;
        let else_branch = &if_else_exp.else_expression;

        assert_eq!(
            condition
                .as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "x"
        );
        assert_eq!(then_branch.as_variable().unwrap().id, "y");
        assert_eq!(else_branch.as_variable().unwrap().id, "z");
    } else {
        panic!("Expected IfElseExpression");
    }
}

#[test]
fn parses_if_else_if_expression() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse("if (x - 5) y else if (y - 7) 4 else 8").unwrap();
    if let Expression::IfElse(if_else_exp) = answ {
        let condition = &if_else_exp.condition;
        let then_branch = &if_else_exp.then_expression;
        let else_branch = &if_else_exp.else_expression;

        assert_eq!(
            condition
                .as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "x"
        );
        assert_eq!(then_branch.as_variable().unwrap().id, "y");
        assert_eq!(
            else_branch
                .as_if_else()
                .unwrap()
                .then_expression
                .as_number_literal()
                .unwrap()
                .value,
            4.0
        );
    } else {
        panic!("Expected IfElseExpression");
    }
}
