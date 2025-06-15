use std::vec;

use crate::parser::Parser;

fn parse_and_get_errors(input: &str) -> Vec<String> {
    let parser = Parser::new();
    match parser.parse(input) {
        Ok(_) => vec![],
        Err(errors) => errors,
    }
}

#[test]
fn test_invalid_token() {
    let errors = parse_and_get_errors("@invalid");
    assert_eq!(
        errors,
        vec![
            "Sintactic Error: Unrecognized token at location: 0, token: `@`, expected: r#``(?:[^`]|.)*``#, r#`[0-9]+(.[0-9]+)?`#, r#`[A-Za-z][A-Za-z_0-9]*`#, `!`, `(`, `+`, `-`, `[`, `constant`, `false`, `for`, `function`, `if`, `let`, `new`, `true`, `type`, `while`, `{`"
        ]
    );
}

#[test]
fn test_unrecognized_eof() {
    let errors = parse_and_get_errors("type Foo {");
    assert_eq!(
        errors,
        vec![
            "Sintactic Error: Unrecognized EOF at location: 10, expected: r#`[A-Za-z][A-Za-z_0-9]*`#, `}`"
        ]
    );
}

#[test]
fn test_unrecognized_token() {
    let errors = parse_and_get_errors("type 123 {");
    assert_eq!(
        errors,
        vec![
            "Sintactic Error: Unrecognized token at location: 5, token: `123`, expected: r#`[A-Za-z][A-Za-z_0-9]*`#"
        ]
    );
}

#[test]
fn test_extra_token() {
    let errors = parse_and_get_errors("type Foo {} extra");
    assert_eq!(
        errors,
        vec![
            "Sintactic Error: Unrecognized EOF at location: 17, expected: `!=`, `%`, `&&`, `(`, `*`, `+`, `-`, `.`, `/`, `:=`, `;`, `<`, `<=`, `==`, `>`, `>=`, `@`, `@@`, `[`, `||`"
        ]
    );
}

#[test]
fn test_multiple_errors() {
    let errors = parse_and_get_errors("type 123 { let = ; }");
    assert_eq!(
        errors,
        vec![
            "Sintactic Error: Unrecognized token at location: 5, token: `123`, expected: r#`[A-Za-z][A-Za-z_0-9]*`#"
        ]
    );
}
