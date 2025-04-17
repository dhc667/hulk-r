use crate::{ast::Atom, tokens::UnaryOperator};

use super::super::grammar;

mod if_else_parser;
mod let_in_parser;
mod while_parser;
mod block_parser;
mod literal_parser;
mod print_parser;

#[test]
fn parses_unary_op() {
    let p = grammar::AtomParser::new();

    let answ1 = p.parse("-123").unwrap();
    if let Atom::UnaryOp(unop) = answ1 {
        let op = &unop.op;
        let rhs = &unop.rhs;

        assert!(matches!(op, UnaryOperator::Minus(_)));
        assert_eq!(rhs.as_number_literal().unwrap().value, 123.0);
    } else {
        panic!("Expected UnaryOp");
    }

    let answ2 = p.parse("+123").unwrap();
    if let Atom::UnaryOp(unop) = answ2 {
        let op = &unop.op;
        let rhs = &unop.rhs;

        assert!(matches!(op, UnaryOperator::Plus(_)));
        assert_eq!(rhs.as_number_literal().unwrap().value, 123.0);
    } else {
        panic!("Expected UnaryOp");
    }

    let answ3 = p.parse("-+123").unwrap();
    if let Atom::UnaryOp(unop) = answ3 {
        let op = &unop.op;
        let rhs = &unop.rhs;

        assert!(matches!(op, UnaryOperator::Minus(_)));
        assert_eq!(
            rhs.as_unary_op()
                .unwrap()
                .rhs
                .as_number_literal()
                .unwrap()
                .value,
            123.0
        );
    } else {
        panic!("Expected UnaryOp");
    }
}
