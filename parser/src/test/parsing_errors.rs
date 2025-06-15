use std::vec;

use error_handler::error_handler::ErrorHandler;

use crate::parser::Parser;

fn parse_and_get_errors(input: &str) -> Vec<String> {
    let mut error_handler = ErrorHandler::new(input);
    let parser = Parser::new();
    match parser.parse(input) {
        Ok(_) => vec![],
        Err(errors) => {
            error_handler.extend_errors(errors);
            error_handler.get_raw_errors()
        }
    }
}

#[test]
fn test_invalid_token() {
    let errors = parse_and_get_errors("@invalid");
    assert_eq!(
        errors,
        vec![
            "Sintactic Error: Unrecognized token `@` found, expected: r#``(?:[^`]|.)*``#, r#`[0-9]+(.[0-9]+)?`#, r#`[A-Za-z][A-Za-z_0-9]*`#, `!`, `(`, `+`, `-`, `[`, `constant`, `false`, `function`, `if`, `let`, `new`, `true`, `type`, `while`, `{`"
        ]
    );
}

#[test]
fn test_unrecognized_eof() {
    let errors = parse_and_get_errors("type Foo {");
    assert_eq!(
        errors,
        vec!["Sintactic Error: Unrecognized EOF found, expected: r#`[A-Za-z][A-Za-z_0-9]*`#, `}`"]
    );
}

#[test]
fn test_unrecognized_token() {
    let errors = parse_and_get_errors("type 123 {");
    assert_eq!(
        errors,
        vec![
            "Sintactic Error: Unrecognized token `123` found, expected: r#`[A-Za-z][A-Za-z_0-9]*`#"
        ]
    );
}

#[test]
fn test_extra_token() {
    let errors = parse_and_get_errors("type Foo {} extra");
    assert_eq!(
        errors,
        vec![
            "Sintactic Error: Unrecognized EOF found, expected: `!=`, `%`, `&&`, `(`, `*`, `+`, `-`, `.`, `/`, `:=`, `;`, `<`, `<=`, `==`, `>`, `>=`, `@`, `@@`, `[`, `||`"
        ]
    );
}

#[test]
fn test_multiple_errors() {
    let errors = parse_and_get_errors("type 123 { let = ; }");
    assert_eq!(
        errors,
        vec![
            "Sintactic Error: Unrecognized token `123` found, expected: r#`[A-Za-z][A-Za-z_0-9]*`#"
        ]
    );
}
