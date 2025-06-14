use ast::typing::BuiltInType;

use crate::ProgramParser;

#[test]
fn simple_type_def() {
    let p = ProgramParser::new();

    let answ = p
        .parse(
            "
type Point {
    x = 0;
    y = PI + 2;

    rotate() => print(\"yeah sure\");
}

let p = Point() in print(p);
",
        )
        .unwrap();

    assert_eq!(answ.definitions[0].as_type_def().unwrap().name.id, "Point");
    assert_eq!(
        answ.definitions[0].as_type_def().unwrap().data_member_defs[0]
            .identifier
            .id,
        "x"
    );
}

#[test]
fn simple_inheritance() {
    let p = ProgramParser::new();

    let answ = p
        .parse(
            "
type Parent(x: String) {
    f(): Number => 3;
    
    s = x;
}

type Child(x: String) inherits Parent(x + \"\\\"hello\\\"\") {
    f(): Number => 4;
}

42;
",
        )
        .unwrap();

    assert_eq!(
        answ.definitions[0].as_type_def().unwrap().parameter_list[0]
            .info
            .ty
            .as_ref()
            .unwrap()
            .as_builtin()
            .unwrap(),
        &BuiltInType::String
    );

    assert_eq!(
        answ.definitions[1]
            .as_type_def()
            .unwrap()
            .inheritance_indicator
            .as_ref()
            .unwrap()
            .argument_list[0]
            .as_bin_op()
            .unwrap()
            .rhs
            .as_string_literal()
            .unwrap()
            .string,
        "\"hello\""
    )
}

#[test]
fn no_way_this_works() {
    let p = ProgramParser::new();

    let answ = p
        .parse(
            "
type Sup(x: Number) {
    x = x;
    sup() => x;
}

type NoWayThisWorks(x: Number) inherits Sup(x + 2) {
    yes_it_does(x: Number) => x;
}

let n = NoWayThisWorks(3) in {
    n.yes_it_does(42);
};
",
        )
        .unwrap();

    assert_eq!(
        answ.definitions[0].as_type_def().unwrap().parameter_list[0]
            .info
            .ty
            .as_ref()
            .unwrap()
            .as_builtin()
            .unwrap(),
        &BuiltInType::Number
    );

    assert_eq!(
        answ.definitions[1]
            .as_type_def()
            .unwrap()
            .inheritance_indicator
            .as_ref()
            .unwrap()
            .parent_name
            .id,
        "Sup"
    );
}

#[test]
fn empty_type() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
type Empty {}

let e = Empty() in e;

",
        )
        .unwrap();

    assert_eq!(answ.definitions[0].as_type_def().unwrap().name.id, "Empty");
}
