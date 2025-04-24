use crate::ast;

use crate::grammar;

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
                .as_atom()
                .unwrap()
                .as_identifier()
                .unwrap()
                .id,
            "a"
        );
        assert!(matches!(*right, ast::Expression::Atom(_)))
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

        assert!(matches!(*left, ast::Expression::Atom(_)));

        let b = &(*right)
            .as_atom()
            .unwrap()
            .as_grouped_expression()
            .unwrap()
            .as_bin_op()
            .unwrap()
            .lhs
            .as_atom()
            .unwrap()
            .as_identifier()
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

        assert!(matches!(*left, ast::Expression::Atom(_)));
        assert!(matches!(*right, ast::Expression::Atom(_)));

        let left = *left;
        let left = &left
            .as_atom()
            .unwrap()
            .as_unary_op()
            .unwrap()
            .rhs
            .as_identifier()
            .unwrap()
            .id;
        assert_eq!(left, "a");
    } else {
        panic!("Expected BinOp");
    }
}
