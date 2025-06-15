use ast::typing::BuiltInType;

use crate::ProgramParser;

#[test]
fn arrow_function_def() {
    let p = ProgramParser::new();

    let answ = p
        .parse(
            "
    function f(): Number => 3;
    42;
    ",
        )
        .unwrap();

    assert_eq!(
        answ.definitions[0]
            .as_function_def()
            .unwrap()
            .function_def
            .body
            .as_arrow_expression()
            .unwrap()
            .expression
            .as_number_literal()
            .unwrap()
            .value,
        3.0
    );
}

#[test]
fn arrow_function_def_with_parameters() {
    let p = ProgramParser::new();

    let answ = p
        .parse(
            "
    function f(dog: Dog, n: Number): Number => 3;
    42;
    ",
        )
        .unwrap();

    assert_eq!(
        answ.definitions[0]
            .as_function_def()
            .unwrap()
            .function_def
            .body
            .as_arrow_expression()
            .unwrap()
            .expression
            .as_number_literal()
            .unwrap()
            .value,
        3.0
    );

    assert_eq!(
        answ.definitions[0]
            .as_function_def()
            .unwrap()
            .function_def
            .parameters[0]
            .info
            .ty
            .as_ref()
            .unwrap()
            .as_defined()
            .unwrap()
            .id,
        "Dog"
    );
}

#[test]
fn block_function_def() {
    let p = ProgramParser::new();

    let answ = p
        .parse(
            "
    function f(): Number {3;}
    42;
    ",
        )
        .unwrap();

    assert_eq!(
        answ.definitions[0]
            .as_function_def()
            .unwrap()
            .function_def
            .body
            .as_block()
            .unwrap()
            .body_items[0]
            .as_expression()
            .unwrap()
            .as_number_literal()
            .unwrap()
            .value,
        3.0
    );
}

#[test]
fn iterable_parameter() {
    let p = ProgramParser::new();

    let answ = p
        .parse(
            "
function add(c: Number*, l: Number): Number => let sum = 0, i = 0 in {
    while (i < l) {
        sum := c[i];
    };
    sum;
};

add([1, 2, 3]);

",
        )
        .unwrap();

    assert_eq!(
        answ.definitions[0]
            .as_function_def()
            .unwrap()
            .function_def
            .parameters[0]
            .info
            .ty
            .as_ref()
            .unwrap()
            .as_iterable()
            .unwrap()
            .as_builtin()
            .unwrap(),
        &BuiltInType::Number
    );
}
