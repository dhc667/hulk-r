use crate::{ast::Atom, grammar};

#[test]
fn parses_print_expression() {
    let p = grammar::AtomParser::new();

    let answ = p.parse("print(1 + 2 + 3 * 4)");
    if let Ok(Atom::PrintExpression(print_exp)) = answ {
        let expression = &print_exp.expression;
        assert_eq!(
            expression
                .as_bin_op()
                .unwrap()
                .rhs
                .as_bin_op()
                .unwrap()
                .rhs
                .as_atom()
                .unwrap()
                .as_number_literal()
                .unwrap()
                .value,
            4.0
        );
    } else {
        panic!("Expected PrintExpression");
    }
}

#[test]
fn parses_print_exp_in_then_branch() {
    let p = grammar::AtomParser::new();

    let answ = p.parse("if (x + 3) print(h) else d");
    if let Ok(Atom::IfExpression(if_else_exp)) = answ {
        let condition = &if_else_exp.condition;
        let then_branch = &if_else_exp.then_expression;
        let else_branch = &if_else_exp.else_expression;

        assert_eq!(
            then_branch
                .as_print_expression()
                .unwrap()
                .expression
                .as_atom()
                .unwrap()
                .as_identifier()
                .unwrap()
                .id,
            "h"
        );
        assert_eq!(else_branch.as_identifier().unwrap().id, "d");
    } else {
        panic!("Expected IfElseExpression");
    }
}

#[test]
fn parses_print_exp_in_condition() {
    let p = grammar::AtomParser::new();

    let answ = p.parse("if (print(x + 3)) print(h) else d");
    if let Ok(Atom::IfExpression(if_else_exp)) = answ {
        let condition = &if_else_exp.condition;
        let then_branch = &if_else_exp.then_expression;
        let else_branch = &if_else_exp.else_expression;

        assert_eq!(
            condition
                .as_atom()
                .unwrap()
                .as_print_expression()
                .unwrap()
                .expression
                .as_bin_op()
                .unwrap()
                .lhs
                .as_atom()
                .unwrap()
                .as_identifier()
                .unwrap()
                .id,
            "x"
        );
        assert_eq!(
            then_branch
                .as_print_expression()
                .unwrap()
                .expression
                .as_atom()
                .unwrap()
                .as_identifier()
                .unwrap()
                .id,
            "h"
        );
        assert_eq!(else_branch.as_identifier().unwrap().id, "d");
    } else {
        panic!("Expected IfElseExpression");
    }
}
