use crate::grammar;
use ast;

#[test]
fn parses_added_terms() {
    let p = grammar::AdditionParser::new();

    let answ = p.parse("a + b + c").unwrap();

    if let ast::Expression::BinOp(binop) = answ {
        let left = binop.lhs;
        let right = binop.rhs;

        assert_eq!(
            left.as_bin_op()
                .unwrap()
                .lhs
                .as_atom()
                .unwrap()
                .as_variable()
                .unwrap()
                .identifier
                .id,
            "a"
        );

        assert_eq!(
            right
                .as_atom()
                .unwrap()
                .as_variable()
                .unwrap()
                .identifier
                .id,
            "c"
        )
    } else {
        panic!("Expected BinOp");
    }
}

#[test]
fn parses_added_terms_with_parentheses() {
    let p = grammar::AdditionParser::new();

    let answ = p.parse("a + (b + c)").unwrap();
    if let ast::Expression::BinOp(binop) = answ {
        let left = binop.lhs;
        let right = binop.rhs;

        assert_eq!(
            left.as_atom().unwrap().as_variable().unwrap().identifier.id,
            "a"
        );

        let right = &(*right)
            .as_atom()
            .unwrap()
            .as_grouped_expression()
            .unwrap()
            .as_bin_op()
            .unwrap()
            .lhs
            .as_atom()
            .unwrap()
            .as_variable()
            .unwrap()
            .identifier
            .id;

        assert_eq!(right, "b");
    } else {
        panic!("Expected BinOp");
    }
}
