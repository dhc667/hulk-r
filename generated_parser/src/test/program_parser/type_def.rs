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

#[test]
fn all_kinds_of_member_defs() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse(
            "
type Test(x: Number, y: Number, w: Number) {
    arrow(x: Number, y: Boolean, z: Object): Number => if (y) x else z;
    block(x: Number, y: Boolean, z: Object): Number {
        if (y) {
            return x;
        } else {
            return z;
        };
    }

    x = x;
    y = 42 + y - w;
}

",
        )
        .unwrap();

    let tydef = answ.definitions.pop().unwrap();
    let tydef = tydef.as_type_def().unwrap();

    let arrow = &tydef.function_member_defs[0];
    assert_eq!(arrow.parameters[1].id, "y");

    let block = &tydef.function_member_defs[1];
    assert_eq!(
        block.body.as_block().unwrap().body_items[0]
            .as_expression()
            .unwrap()
            .as_if_else()
            .unwrap()
            .then_expression
            .as_block()
            .unwrap()
            .body_items[0]
            .as_return_statement()
            .unwrap()
            .expression
            .as_variable()
            .unwrap()
            .id,
        "x"
    );

    let w = &tydef.data_member_defs[1]
        .default_value
        .as_bin_op()
        .unwrap()
        .rhs
        .as_variable()
        .unwrap()
        .id;
    assert_eq!(w, "w");
}

#[test]
fn doesnt_recognize_object_as_builtin() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
type Object {}
let a = new Object() in a;
",
        )
        .unwrap();

    assert_eq!(answ.definitions[0].as_type_def().unwrap().name.id, "Object")
}
