use crate::ProgramParser;

#[test]
#[should_panic]
#[ignore = "protocols are disabled"]
fn empty_protocol() {
    let p = ProgramParser::new();
    p.parse(
        "
protocol P {}

42;
",
    )
    .unwrap();
}
