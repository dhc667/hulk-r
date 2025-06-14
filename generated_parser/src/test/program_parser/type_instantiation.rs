use crate::ProgramParser;

#[test]
fn simple_instantiation() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
let a = new Example() in print(a);
",
        )
        .unwrap();

    assert_eq!(
        answ.expressions[0]
            .as_let_in()
            .unwrap()
            .assignment
            .rhs
            .as_new_expression()
            .unwrap()
            .type_name,
        "Example"
    )
}

#[test]
fn argumented_instantiation() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
let a = new Example(1, 1 + 2, let x = 3 in x) in print(a);
",
        )
        .unwrap();

    let args = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .rhs
        .as_new_expression()
        .unwrap()
        .arguments;

    assert_eq!(args[0].as_number_literal().unwrap().value, 1.0);
    assert_eq!(
        args[1]
            .as_bin_op()
            .unwrap()
            .rhs
            .as_number_literal()
            .unwrap()
            .value,
        2.0
    );
    assert_eq!(
        args[2]
            .as_let_in()
            .unwrap()
            .assignment
            .rhs
            .as_number_literal()
            .unwrap()
            .value,
        3.0
    );
}
