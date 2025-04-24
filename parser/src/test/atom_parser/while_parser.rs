use ast;
use crate::grammar;

#[test]
pub fn parses_while_expression() {
    let p = grammar::AtomParser::new();

    let answ = p.parse("while (x - 10) { print(x); x + 2;; }");
    assert!(answ.is_ok());
    if let Ok(ast::Atom::While(while_exp)) = answ {
        let condition = &while_exp.condition;
        let body = &while_exp.body;

        assert_eq!(
            condition
                .as_bin_op()
                .unwrap()
                .lhs
                .as_atom()
                .unwrap()
                .as_identifier()
                .unwrap()
                .id,
            "x"
        );

        assert_eq!(body.as_block().unwrap().expression_list.expressions.len(), 2);

        assert!(body.as_block().unwrap().expression_list.multiple_semicolon_terminated,);

        assert_eq!(
            body.as_block().unwrap().expression_list.expressions[1]
                .as_bin_op()
                .unwrap()
                .lhs
                .as_atom()
                .unwrap()
                .as_identifier()
                .unwrap()
                .id,
            "x"
        )
    } else {
        panic!("Expected WhileExpression");
    }
}
