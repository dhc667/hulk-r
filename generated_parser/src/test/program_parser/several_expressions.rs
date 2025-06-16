use crate::ProgramParser;

#[test]
fn several_expressions() {
    let p = ProgramParser::new();

    let answ = p
        .parse(
            "
    print(1);
    print(2);
",
        )
        .unwrap();

    assert_eq!(answ.expressions.len(), 2);
    assert_eq!(
        answ.expressions[0].as_function_call().unwrap().arguments[0]
            .as_number_literal()
            .unwrap()
            .value,
        1.0
    );
    assert_eq!(
        answ.expressions[1].as_function_call().unwrap().arguments[0]
            .as_number_literal()
            .unwrap()
            .value,
        2.0
    );
}
