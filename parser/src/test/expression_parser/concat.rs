use crate::grammar;

#[test]
fn simple_concat() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse("a @@ b @ c").unwrap();

    assert_eq!(
        answ.as_bin_op()
            .unwrap()
            .lhs
            .as_bin_op()
            .unwrap()
            .rhs
            .as_variable()
            .unwrap()
            .id,
        "b"
    )
}

#[test]
fn concat_is_lowest_precedence() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse("a + b @ c || d").unwrap();

    assert!(matches!(
        answ.as_bin_op()
            .unwrap()
            .op,
        ast::BinaryOperator::At(_)
    ))
}

#[test]
fn concat_is_lowest_precedence_2() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse("a + b @ c || d @@ c").unwrap();

    assert!(matches!(
        answ.as_bin_op()
            .unwrap()
            .op,
        ast::BinaryOperator::AtAt(_)
    ))
}
