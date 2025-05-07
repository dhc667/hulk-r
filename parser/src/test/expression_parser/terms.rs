use crate::grammar;
use ast::{self, Expression};

#[test]
fn parses_term() {
    let p = grammar::TermParser::new();

    let answ = p.parse("a * b / c").unwrap();
    if let ast::Expression::BinOp(binop) = answ {
        let left = binop.lhs;
        let right = binop.rhs;
        assert_eq!(
            left.as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "a"
        );
        assert!(matches!(*right, Expression::Variable(_)))
    } else {
        panic!("Expected BinOp");
    }
}

#[test]
fn parses_term_with_parentheses() {
    let p = grammar::TermParser::new();

    let answ = p.parse("a * (b / c)").unwrap();
    if let ast::Expression::BinOp(binop) = answ {
        let left = binop.lhs;
        let right = binop.rhs;

        assert!(matches!(*left, Expression::Variable(_)));

        let b = &(*right)
            .as_bin_op()
            .unwrap()
            .lhs
            .as_variable()
            .unwrap()
            .id;
        assert_eq!(b, "b");
    } else {
        panic!("Expected BinOp");
    }
}

#[test]
fn parses_term_with_unary_operator() {
    let p = grammar::TermParser::new();

    let answ = p.parse("-a * b").unwrap();
    if let ast::Expression::BinOp(binop) = answ {
        let left = binop.lhs;
        let right = binop.rhs;

        assert!(matches!(*left, Expression::UnaryOp(_)));
        assert!(matches!(*right, Expression::Variable(_)));

        let left = *left;
        let left = &left
            .as_unary_op()
            .unwrap()
            .rhs
            .as_variable()
            .unwrap()
            .id;
        assert_eq!(left, "a");
    } else {
        panic!("Expected BinOp");
    }
}
