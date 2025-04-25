use crate::grammar;
use ast::BooleanLiteral;

#[test]
fn parses_and_or() {
    let p = grammar::ExpressionParser::new();
    let ast = p.parse("a || b && c").unwrap();

    assert_eq!(
        ast.as_bin_op()
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
        ast.as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .rhs
            .as_atom()
            .unwrap()
            .as_variable()
            .unwrap()
            .identifier
            .id,
        "c"
    )
}

#[test]
fn parses_leq_get_eq() {
    let p = grammar::ExpressionParser::new();
    let ast = p.parse("a || 4 < 3 == 3 < 4").unwrap();

    assert_eq!(
        ast.as_bin_op()
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
        ast.as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .lhs
            .as_bin_op()
            .unwrap()
            .lhs
            .as_atom()
            .unwrap()
            .as_number_literal()
            .unwrap()
            .value,
        4.0
    );
}

#[test]
fn logical_literals() {
    let p = grammar::ExpressionParser::new();
    let ast = p.parse("true || false && 3 < 4").unwrap();

    assert!(matches!(
        ast.as_bin_op()
            .unwrap()
            .lhs
            .as_atom()
            .unwrap()
            .as_boolean_literal()
            .unwrap(),
        BooleanLiteral::True(_)
    ));

    assert!(matches!(
        ast.as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .lhs
            .as_atom()
            .unwrap()
            .as_boolean_literal()
            .unwrap(),
        BooleanLiteral::False(_)
    ));
}
