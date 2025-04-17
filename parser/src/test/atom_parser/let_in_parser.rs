use crate::ast::Atom;
use crate::grammar;

#[test]
fn parses_let_in_expression() {
    let p = grammar::AtomParser::new();

    let answ = p.parse("let x = 5 in (x + 1)").unwrap();
    if let Atom::LetExpression(let_exp) = answ {
        let assignment_list = &let_exp.assignments;
        assert_eq!(assignment_list.len(), 1);
        assert_eq!(&assignment_list[0].identifier.id, "x");

        let body = &let_exp.body;

        let x = &body
            .as_grouped_expression()
            .unwrap()
            .as_bin_op()
            .unwrap()
            .lhs
            .as_atom()
            .unwrap()
            .as_identifier()
            .unwrap()
            .id;

        assert_eq!(x, "x");
    } else {
        panic!("Expected LetIn");
    }
}

#[test]
fn parses_let_in_exp_with_several_assignments() {
    let p = grammar::AtomParser::new();

    let answ = p.parse("let x = 5, y = 10 in (x + y)").unwrap();
    if let Atom::LetExpression(let_exp) = answ {
        let assignment_list = &let_exp.assignments;
        assert_eq!(assignment_list.len(), 2);
        assert_eq!(&assignment_list[0].identifier.id, "x");
        assert_eq!(&assignment_list[1].identifier.id, "y");

        let body = &let_exp.body;
        let x = &body
            .as_grouped_expression()
            .unwrap()
            .as_bin_op()
            .unwrap()
            .lhs
            .as_atom()
            .unwrap()
            .as_identifier()
            .unwrap()
            .id;

        assert_eq!(x, "x");
    } else {
        panic!("Expected LetIn");
    }
}

#[test]
fn parses_let_in_exp_with_single_variable_as_output() {
    let p = grammar::AtomParser::new();

    let answ = p.parse("let x = 5 in x").unwrap();
    if let Atom::LetExpression(let_exp) = answ {
        let assignment_list = &let_exp.assignments;
        assert_eq!(assignment_list.len(), 1);
        assert_eq!(&assignment_list[0].identifier.id, "x");

        let body = &let_exp.body;
        let x = &body.as_identifier().unwrap().id;

        assert_eq!(x, "x");
    } else {
        panic!("Expected LetIn");
    }
}
