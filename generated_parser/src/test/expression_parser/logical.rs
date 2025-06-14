use ast::BooleanLiteral;

use crate::test::expression_parser::ExpressionParser;

#[test]
fn parses_and_or() {
    let p = ExpressionParser::new();
    let ast = p.parse("a || b && c").unwrap();

    assert_eq!(ast.as_bin_op().unwrap().lhs.as_variable().unwrap().id, "a");

    assert_eq!(
        ast.as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .rhs
            .as_variable()
            .unwrap()
            .id,
        "c"
    )
}

#[test]
fn parses_leq_get_eq() {
    let p = ExpressionParser::new();
    let ast = p.parse("a || 4 < 3 == 3 < 4").unwrap();

    assert_eq!(ast.as_bin_op().unwrap().lhs.as_variable().unwrap().id, "a");

    assert_eq!(
        ast.as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .lhs
            .as_bin_op()
            .unwrap()
            .lhs
            .as_number_literal()
            .unwrap()
            .value,
        4.0
    );
}

#[test]
fn logical_literals() {
    let p = ExpressionParser::new();
    let ast = p.parse("true || false && 3 < 4").unwrap();

    assert!(matches!(
        ast.as_bin_op().unwrap().lhs.as_boolean_literal().unwrap(),
        BooleanLiteral::True(_)
    ));

    assert!(matches!(
        ast.as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .lhs
            .as_boolean_literal()
            .unwrap(),
        BooleanLiteral::False(_)
    ));
}
