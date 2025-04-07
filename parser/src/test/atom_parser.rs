use super::super::grammar;
use super::super::ast::{Atom, NumberLiteral, Identifier, Operator};

#[test]
fn parses_number_literals() {
    let p = grammar::AtomParser::new();

    let answ1 = p.parse("123");
    assert!(matches!(answ1.unwrap(), Atom::NumberLiteral(NumberLiteral(_, 123.0))));

    let answ2 = p.parse("123.456");
    assert!(matches!(answ2.unwrap(), Atom::NumberLiteral(NumberLiteral(_, 123.456))));
}

#[test]
fn detects_number_literal_error() {
    let p = grammar::FactorParser::new();

    let answ1 = p.parse("123.456.789");
    assert!(answ1.is_err());

    let answ2 = p.parse("123a");
    assert!(answ2.is_err());

    let answ3 = p.parse("123.456a");
    assert!(answ3.is_err());
}

#[test]
fn parses_identifier() {
    let p = grammar::AtomParser::new();

    let answ1 = p.parse("abc").unwrap();
    if let Atom::Identifier(Identifier(_, id)) = answ1 {
        assert_eq!(id, "abc");
    } else {
        panic!("Expected Identifier");
    }


    let answ2 = p.parse("abc123").unwrap();
    if let Atom::Identifier(Identifier(_, id)) = answ2 {
        assert_eq!(id, "abc123");
    } else {
        panic!("Expected Identifier");
    }

    let answ3 = p.parse("abc_123").unwrap();
    if let Atom::Identifier(Identifier(_, id)) = answ3 {
        assert_eq!(id, "abc_123");
    } else {
        panic!("Expected Identifier");
    }
}

#[test]
fn detects_identifier_error() {
    let p = grammar::FactorParser::new();

    let answ1 = p.parse("123abc");
    assert!(answ1.is_err());

    let answ2 = p.parse("abc-123");
    assert!(answ2.is_err());

    let answ3 = p.parse("abc.123");
    assert!(answ3.is_err());
}

#[test]
fn parses_unary_op() {
    let p = grammar::AtomParser::new();

    let answ1 = p.parse("-123").unwrap();
    if let Atom::UnaryOp(op, factor) = answ1 {
        assert!(matches!(op, Operator::Minus(_)));
        assert!(matches!(*factor, Atom::NumberLiteral(NumberLiteral(_, 123.0))));
    } else {
        panic!("Expected UnaryOp");
    }

    let answ2 = p.parse("+123").unwrap();
    if let Atom::UnaryOp(op, factor) = answ2 {
        assert!(matches!(op, Operator::Plus(_)));
        assert!(matches!(*factor, Atom::NumberLiteral(NumberLiteral(_, 123.0))));
    } else {
        panic!("Expected UnaryOp");
    }

    let answ3 = p.parse("-+123").unwrap();
    if let Atom::UnaryOp(op, factor) = answ3 {
        assert!(matches!(op, Operator::Minus(_)));
        if let Atom::UnaryOp(op2, factor2) = *factor {
            assert!(matches!(op2, Operator::Plus(_)));
            assert!(matches!(*factor2, Atom::NumberLiteral(NumberLiteral(_, 123.0))));
        } else {
            panic!("Expected UnaryOp");
        }
    } else {
        panic!("Expected UnaryOp");
    }
}
