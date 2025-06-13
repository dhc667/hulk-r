use crate::ProgramParser;

#[test]
#[should_panic]
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
