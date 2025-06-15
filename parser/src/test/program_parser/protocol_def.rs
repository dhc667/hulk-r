use ast::typing::BuiltInType;

use crate::ProgramParser;

#[test]
#[ignore = "protocols are disabled"]
fn simple_protocol() {
    let p = ProgramParser::new();

    let answ = p
        .parse(
            "

protocol Hash {
    hash(o: String): Number;
}

42;

",
        )
        .unwrap();

    assert_eq!(
        answ.definitions[0]
            .as_protocol_def()
            .unwrap()
            .function_signatures[0]
            .parameters[0]
            .info
            .ty
            .as_ref()
            .unwrap()
            .as_builtin()
            .unwrap(),
        &BuiltInType::String
    )
}
