use crate::grammar::ExpressionParser;
use ast;

#[test]
fn parses_added_terms() {
    let p = ExpressionParser::new();

    let answ = p.parse("a + b + c").unwrap();

    if let ast::Expression::BinOp(binop) = answ {
        let left = binop.lhs;
        let right = binop.rhs;

        assert_eq!(left.as_bin_op().unwrap().lhs.as_variable().unwrap().id, "a");

        assert_eq!(right.as_variable().unwrap().id, "c")
    } else {
        panic!("Expected BinOp");
    }
}

#[test]
fn parses_added_terms_with_parentheses() {
    let p = ExpressionParser::new();

    let answ = p.parse("a + (b + c)").unwrap();
    if let ast::Expression::BinOp(binop) = answ {
        let left = binop.lhs;
        let right = binop.rhs;

        assert_eq!(left.as_variable().unwrap().id, "a");

        let right = &(*right).as_bin_op().unwrap().lhs.as_variable().unwrap().id;

        assert_eq!(right, "b");
    } else {
        panic!("Expected BinOp");
    }
}

#[test]
fn adding_with_string() {
    let p = ExpressionParser::new();

    let answ = p.parse("\"test\" + b * (\"test 2\" - 2)").unwrap();

    assert_eq!(
        answ.as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .lhs
            .as_string_literal()
            .unwrap()
            .string,
        "test 2"
    );

    assert_eq!(
        answ.as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .lhs
            .as_variable()
            .unwrap()
            .id,
        "b"
    );
}
