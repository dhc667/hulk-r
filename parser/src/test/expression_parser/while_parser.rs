use crate::grammar;
use ast;

#[test]
pub fn parses_while_expression() {
    let p = grammar::ExpressionParser::new();

    let answ = p.parse("while (x - 10) { print(x); x + 2;; }");
    assert!(answ.is_ok());
    if let Ok(ast::Expression::While(while_exp)) = answ {
        let condition = &while_exp.condition;
        let body = &while_exp.body;

        assert_eq!(
            condition.as_bin_op().unwrap().lhs.as_variable().unwrap().id,
            "x"
        );

        assert_eq!(body.body.body_items.len(), 2);

        assert!(body.body.multiple_semicolon_terminated,);

        assert_eq!(
            body.body.body_items[1]
                .as_expression()
                .unwrap()
                .as_bin_op()
                .unwrap()
                .lhs
                .as_variable()
                .unwrap()
                .id,
            "x"
        )
    } else {
        panic!("Expected WhileExpression");
    }
}
