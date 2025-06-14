use crate::test::expression_parser::ExpressionParser;


#[test]
fn simple_for() {
    let p = ExpressionParser::new();

    let answ = p.parse("for (x in y) {print(x);}").unwrap();

    assert_eq!(answ.as_for().unwrap().element.id, "x");
    assert_eq!(
        answ.as_for().unwrap().iterable.as_variable().unwrap().id,
        "y"
    );
    assert_eq!(
        answ.as_for().unwrap().body.body_items[0]
            .as_expression()
            .unwrap()
            .as_function_call()
            .unwrap()
            .arguments[0]
            .as_variable()
            .unwrap()
            .id,
        "x"
    );
}

#[test]
fn complex_iterable() {
    let p = ExpressionParser::new();

    let answ = p
        .parse(
            "
        for (x in
            [1, 2, 3] + [4, 5] * q([\"woof\"])
        ) {
            print(\"woof\");
        }",
        )
        .unwrap();

    assert_eq!(
        answ.as_for()
            .unwrap()
            .iterable
            .as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .rhs
            .as_function_call()
            .unwrap()
            .arguments[0]
            .as_list_literal()
            .unwrap()
            .elements[0]
            .as_string_literal()
            .unwrap()
            .string,
        "woof"
    )
}
