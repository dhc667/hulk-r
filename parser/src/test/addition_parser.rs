use crate::ast::Operator;

use super::super::grammar;
use super::super::ast;

#[test]
fn parses_added_terms() {
    let p = grammar::AdditionParser::new();

    let answ1 = p.parse("a + b + c");
    assert!(answ1.is_ok());
    let term = answ1.unwrap();
    if let ast::Addition::BinaryOp(left, _, right) = term {
        let left = *left;
        assert!(matches!(left, ast::Addition::BinaryOp(_, ast::Operator::Plus(_), _)));

        let right = *right;
        assert!(matches!(right, ast::Term::Factor(_)));
    } else {
        panic!("Expected BinaryOp");
    }

    let answ2 = p.parse("a + b");
    assert!(answ2.is_ok());
    let term = answ2.unwrap();
    if let ast::Addition::BinaryOp(left, op, right) = term {
        match *left {
            ast::Addition::Term(term_box) => {
                match *term_box {
                    ast::Term::Factor(factor_box) => {
                        match *factor_box {
                            ast::Factor::Atom(atom) => {
                                assert!(matches!(atom, ast::Atom::Identifier(_)));
                            }
                            _ => panic!("Expected Factor"),
                        }
                    }
                    _ => panic!("Expected Factor"),
                }
            }
            _ => panic!("Expected Factor"),
        }
        assert!(matches!(op, ast::Operator::Plus(_)));
        let right = *right;
        assert!(matches!(right, ast::Term::Factor(_)));
    } else {
        panic!("Expected BinaryOp");
    }
}

#[test]
fn parses_added_terms_with_unary_op() {
    let p = grammar::AdditionParser::new();

    let answ1 = p.parse("a + -b + c");
    assert!(answ1.is_ok());
    let term = answ1.unwrap();
    if let ast::Addition::BinaryOp(left, _, right) = term {
        let left = *left;
        assert!(matches!(left, ast::Addition::BinaryOp(_, ast::Operator::Plus(_), _)));
        if let ast::Addition::BinaryOp(_, _, right) = left {
            assert!(matches!(*right, ast::Term::Factor(_)));
            let right = *right;
            if let ast::Term::Factor(right) = right {
                assert!(matches!(*right, ast::Factor::Atom(ast::Atom::UnaryOp(Operator::Minus(_), _))))
            }
            else {
                panic!("Expected Factor");
            }
        } else {
            panic!("Expected BinaryOp");
        }

        let right = *right;
        assert!(matches!(right, ast::Term::Factor(_)));
    } else {
        panic!("Expected BinaryOp");
    }
}

