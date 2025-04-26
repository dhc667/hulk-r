use ast::Visitable;
use parser::ProgramParser;

use crate::SemanticVisitor;

#[test]
fn global_context() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x = 1 + 2 in {x + 2;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.accept(&mut semantic_visitor);
    let let_in = answ.expression_list.expressions[0]
        .as_atom()
        .unwrap()
        .as_let_expression()
        .unwrap();
    let initializer_id = &let_in.assignment.identifier;
    let sum = let_in.body.as_block().unwrap().expression_list.expressions[0]
        .as_bin_op()
        .unwrap();
    let left = sum.lhs.as_atom().unwrap().as_identifier().unwrap();

    assert_eq!(initializer_id.context_id, Some(1));
    assert_eq!(left.context_id, Some(1));
}

#[test]
fn nested_context() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("let x = 1 + 2 in {let y = x + 2 in {y + 2;};};")
        .unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.accept(&mut semantic_visitor);
    let let_in = answ.expression_list.expressions[0]
        .as_atom()
        .unwrap()
        .as_let_expression()
        .unwrap();
    let x_initializer_id = &let_in.assignment.identifier;

    let inner_let = let_in.body.as_block().unwrap().expression_list.expressions[0]
        .as_atom()
        .unwrap()
        .as_let_expression()
        .unwrap();
    let y_initializer_id = &inner_let.assignment.identifier;

    let inner_def_expr = inner_let.assignment.rhs.as_bin_op().unwrap();
    let x_inner = inner_def_expr
        .lhs
        .as_atom()
        .unwrap()
        .as_identifier()
        .unwrap();

    let inner_sum = inner_let
        .body
        .as_block()
        .unwrap()
        .expression_list
        .expressions[0]
        .as_bin_op()
        .unwrap();
    let y_call_id = inner_sum.lhs.as_atom().unwrap().as_identifier().unwrap();

    assert_eq!(x_initializer_id.context_id, Some(1));
    assert_eq!(x_inner.context_id, Some(1));

    assert_eq!(y_initializer_id.context_id, Some(2));
    assert_eq!(y_call_id.context_id, Some(2));
}

#[test]
fn uninitialized_variable() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x = x in {x + 2;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.accept(&mut semantic_visitor);

    assert_eq!(
        semantic_visitor.errors,
        vec!["Cannot read variable x in its own initializer".to_string()]
    );
}
