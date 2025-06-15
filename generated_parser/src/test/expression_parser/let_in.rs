use crate::test::expression_parser::ExpressionParser;
use ast::Expression;

#[test]
fn parses_let_in_expression() {
    let p = ExpressionParser::new();

    let answ = p.parse("let x = 5 in (x + 1)").unwrap();
    if let Expression::LetIn(let_exp) = answ {
        let assignment = &let_exp.assignment;
        assert_eq!(&assignment.identifier.id, "x");

        let body = &let_exp.body;

        let x = &body.as_bin_op().unwrap().lhs.as_variable().unwrap().id;

        assert_eq!(x, "x");
    } else {
        panic!("Expected LetIn");
    }
}

#[test]
fn parses_let_in_exp_with_several_assignments() {
    let p = ExpressionParser::new();

    let answ = p.parse("let x = 5, y = 10 in (x + y)").unwrap();
    if let Expression::LetIn(let_exp) = answ {
        let first_assignment = &let_exp.assignment;
        assert_eq!(first_assignment.identifier.id, "x");

        let second_assignment = &let_exp.body.as_let_in().unwrap().assignment;
        assert_eq!(second_assignment.identifier.id, "y");

        let body = &let_exp.body.as_let_in().unwrap().body;
        let x = &body.as_bin_op().unwrap().lhs.as_variable().unwrap().id;

        assert_eq!(x, "x");
    } else {
        panic!("Expected LetIn");
    }
}

#[test]
fn parses_let_in_exp_with_single_variable_as_output() {
    let p = ExpressionParser::new();

    let answ = p.parse("let x = 5 in x").unwrap();
    if let Expression::LetIn(let_exp) = answ {
        let assignment = &let_exp.assignment;
        assert_eq!(&assignment.identifier.id, "x");

        let body = &let_exp.body;
        let x = &body.as_variable().unwrap().id;

        assert_eq!(x, "x");
    } else {
        panic!("Expected LetIn");
    }
}
