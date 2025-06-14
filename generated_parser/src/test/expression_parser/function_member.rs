use crate::test::expression_parser::ExpressionParser;


#[test]
fn simple_object_function() {
    let p = ExpressionParser::new();

    let answ = p.parse("o.foo(x)").unwrap();

    assert_eq!(
        answ.as_function_member_access()
            .unwrap()
            .object
            .as_variable()
            .unwrap()
            .id,
        "o"
    );

    assert_eq!(
        answ.as_function_member_access()
            .unwrap()
            .member
            .identifier
            .id,
        "foo"
    );

    assert_eq!(
        answ.as_function_member_access().unwrap().member.arguments[0]
            .as_variable()
            .unwrap()
            .id,
        "x"
    );
}

#[test]
fn complex_object_function() {
    let p = ExpressionParser::new();

    let answ = p.parse("{a; b; c;}.foo(x)").unwrap();

    assert_eq!(
        answ.as_function_member_access()
            .unwrap()
            .object
            .as_block()
            .unwrap()
            .body_items[2]
            .as_expression()
            .unwrap()
            .as_variable()
            .unwrap()
            .id,
        "c"
    );

    assert_eq!(
        answ.as_function_member_access()
            .unwrap()
            .member
            .identifier
            .id,
        "foo"
    );
}
