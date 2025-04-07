use crate::ast;

use super::super::grammar;

#[test]
fn parses_term() {
    let p = grammar::TermParser::new();

    let answ1 = p.parse("a*b/c*d");
    assert!(answ1.is_ok());
    let term = answ1.unwrap();
    if let ast::Term::BinaryOp(left, op, right) = term {
        assert!(matches!(*left, ast::Term::BinaryOp(_, ast::Operator::Divide(_), _)));
        assert!(matches!(op, ast::Operator::Times(_)));
        assert!(matches!(*right, ast::Factor::Atom(ast::Atom::Identifier(_))));
    } else {
        panic!("Expected BinaryOp");
    }


    let answ2 = p.parse("a*b");
    assert!(answ2.is_ok());
    let term = answ2.unwrap();
    if let ast::Term::BinaryOp(left, op, right) = term {
        match *left {
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
        assert!(matches!(op, ast::Operator::Times(_)));
        assert!(matches!(*right, ast::Factor::Atom(ast::Atom::Identifier(_))));
    } else {
        panic!("Expected BinaryOp");
    }
}

