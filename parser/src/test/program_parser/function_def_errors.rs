use crate::ProgramParser;

#[test]
#[should_panic]
fn arrow_func_def_missing_semicolon() {
    let p = ProgramParser::new();

    p.parse(
        "
    function f(): number => 3
    42;
",
    )
    .unwrap();
}

#[test]
#[should_panic]
fn arrow_function_missing_type_annotation() {
    let p = ProgramParser::new();

    p.parse(
        "
    function f() => 3;
    42;
",
    )
    .unwrap();
}

#[test]
#[should_panic]
fn block_func_def_with_semicolon() {
    let p = ProgramParser::new();

    p.parse("function f(): number {a;}; 42;").unwrap();
}

#[test]
#[should_panic]
fn parameter_missing_type_annotation() {
    let p = ProgramParser::new();

    p.parse("function(x): number => x;").unwrap();
}

#[test]
#[should_panic]
fn some_parameter_missing_type_annotation() {
    let p = ProgramParser::new();

    p.parse("function(x: number, y, z: Dog): number => x;").unwrap();
}
