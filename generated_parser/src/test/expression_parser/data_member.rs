use crate::test::expression_parser::ExpressionParser;


#[test]
fn simple_object_data_access() {
    let p = ExpressionParser::new();

    let answ = p.parse("x.y").unwrap();

    assert_eq!(
        answ.as_data_member_access()
            .unwrap()
            .object
            .as_variable()
            .unwrap()
            .id,
        "x"
    )
}

#[test]
fn complex_object_data_access() {
    let p = ExpressionParser::new();

    let answ = p.parse("{a; x;}.y").unwrap();

    assert_eq!(
        answ.as_data_member_access()
            .unwrap()
            .object
            .as_block()
            .unwrap()
            .body_items[1]
            .as_expression()
            .unwrap()
            .as_variable()
            .unwrap()
            .id,
        "x"
    );
}
