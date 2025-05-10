use crate::ProgramParser;

#[test]
fn basic_constant() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
constant PI: Number = 3.14;

PI;
",
        )
        .unwrap();

    assert_eq!(
        answ.definitions[0]
            .as_constant_def()
            .unwrap()
            .initializer_expression
            .as_number_literal()
            .unwrap()
            .value,
        3.14
    );
}

#[test]
fn complex_constant_initializer() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
constant PI: Number = 22 / 7 * 100 / (50 + 50);
constant PI2: Number = calculate_pi();

PI + P2;
",
        )
        .unwrap();

    assert_eq!(
        answ.definitions[0]
            .as_constant_def()
            .unwrap()
            .initializer_expression
            .as_bin_op()
            .unwrap()
            .rhs
            .as_bin_op()
            .unwrap()
            .lhs
            .as_number_literal()
            .unwrap()
            .value,
        50.0
    );

    assert_eq!(
        answ.definitions[1]
            .as_constant_def()
            .unwrap()
            .initializer_expression
            .as_function_call()
            .unwrap()
            .identifier
            .id,
        "calculate_pi"
    );
}
