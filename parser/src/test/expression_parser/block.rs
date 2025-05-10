use crate::grammar::{self, ExpressionParser};
use ast;

#[test]
fn detects_single_semicolon_terminated_block() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse(
        "{
a + b;
c + d;;;
x - 4;
}",
    );

    if let Ok(ast::Expression::Block(block)) = answ {
        assert_eq!(block.body.body_items.len(), 3);
        assert!(!block.body.multiple_semicolon_terminated);
        assert_eq!(
            block.body.body_items[0]
                .as_expression()
                .unwrap()
                .as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "a"
        );
        assert_eq!(
            block.body.body_items[1]
                .as_expression()
                .unwrap()
                .as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "c"
        );
        assert_eq!(
            block.body.body_items[2]
                .as_expression()
                .unwrap()
                .as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "x"
        );
    } else {
        panic!("Expected Block");
    }
}

#[test]
fn detects_multiple_semicolon_terminated_block() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse(
        "{
a + b;
c + d;
x - 4 + 6 / (2 + 3 - x);;
}",
    );

    if let Ok(ast::Expression::Block(block)) = answ {
        assert_eq!(block.body.body_items.len(), 3);
        assert!(block.body.multiple_semicolon_terminated);
        assert_eq!(
            block.body.body_items[0]
                .as_expression()
                .unwrap()
                .as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "a"
        );
        assert_eq!(
            block.body.body_items[1]
                .as_expression()
                .unwrap()
                .as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "c"
        );
        assert_eq!(
            block.body.body_items[2]
                .as_expression()
                .unwrap()
                .as_bin_op()
                .unwrap()
                .rhs
                .as_bin_op()
                .unwrap()
                .rhs
                .as_bin_op()
                .unwrap()
                .rhs
                .as_variable()
                .unwrap()
                .id,
            "x"
        );
    } else {
        panic!("Expected Block");
    }
}

#[test]
fn detects_return_statements() {
    let p = ExpressionParser::new();

    let answ = p
        .parse(
            "let a = 3 in {
    a := a + 1;
    a := a + 2;
    if (a == 3) {return a;} else 3;
    a := 5;
}",
        )
        .unwrap();

    assert_eq!(
        answ.as_let_in()
            .unwrap()
            .body
            .as_block()
            .unwrap()
            .body
            .body_items[2]
            .as_expression()
            .unwrap()
            .as_if_else()
            .unwrap()
            .then_expression
            .as_block()
            .unwrap()
            .body
            .body_items[0]
            .as_return_statement()
            .unwrap()
            .expression
            .as_variable()
            .unwrap()
            .id,
        "a"
    );
}
