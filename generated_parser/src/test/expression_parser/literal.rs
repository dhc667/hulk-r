use ast::Expression;

use crate::test::expression_parser::ExpressionParser;

#[test]
fn parses_number_literals() {
    let p = ExpressionParser::new();

    let answ1 = p.parse("123");
    if let Ok(Expression::NumberLiteral(num)) = answ1 {
        assert_eq!(num.value, 123.0);
    } else {
        panic!("Expected NumberLiteral");
    }

    let answ2 = p.parse("123.456");
    if let Ok(Expression::NumberLiteral(num)) = answ2 {
        assert_eq!(num.value, 123.456);
    } else {
        panic!("Expected NumberLiteral");
    }
}

#[test]
fn detects_number_literal_error() {
    let p = ExpressionParser::new();

    let answ1 = p.parse("123.456.789");
    assert!(answ1.is_err());

    let answ2 = p.parse("123a");
    assert!(answ2.is_err());

    let answ3 = p.parse("123.456a");
    assert!(answ3.is_err());
}

#[test]
fn parses_identifier() {
    let p = ExpressionParser::new();

    let answ1 = p.parse("abc").unwrap();
    if let Expression::Variable(identifier) = answ1 {
        assert_eq!(identifier.id, "abc");
    } else {
        panic!("Expected Identifier");
    }

    let answ2 = p.parse("abc123").unwrap();
    if let Expression::Variable(identifier) = answ2 {
        assert_eq!(identifier.id, "abc123");
    } else {
        panic!("Expected Identifier");
    }

    let answ3 = p.parse("abc_123").unwrap();
    if let Expression::Variable(identifier) = answ3 {
        assert_eq!(identifier.id, "abc_123");
    } else {
        panic!("Expected Identifier");
    }
}

#[test]
fn detects_identifier_error() {
    let p = ExpressionParser::new();

    let answ1 = p.parse("123abc");
    assert!(answ1.is_err());

    let answ2 = p.parse("abc-123").unwrap();
    assert!(matches!(answ2, Expression::BinOp(_)));

    let answ3 = p.parse("abc.123");
    assert!(answ3.is_err());
}

#[test]
fn string_literal() {
    let p = ExpressionParser::new();

    let answ = p.parse("\"test\"").unwrap();

    assert_eq!(answ.as_string_literal().unwrap().string, "test");
}
