use crate::test::expression_parser::ExpressionParser;

#[test]
fn parses_string() {
    let p = ExpressionParser::new();

    let answ = p.parse(r#" "hello there mes amis" "#).unwrap();

    assert_eq!(
        answ.as_string_literal().unwrap().string,
        "hello there mes amis"
    );
}

#[test]
fn parses_escaped_string() {
    let p = ExpressionParser::new();
    let s = r#" "hello there mes amis, tout va \"bien\"?" "#;

    let answ = p.parse(s).unwrap();

    assert_eq!(
        answ.as_string_literal().unwrap().string,
        r#"hello there mes amis, tout va "bien"?"#
    );
}
