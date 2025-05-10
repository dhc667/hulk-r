use ast::{
    VisitableExpression,
    typing::{BuiltInType, Type},
};
use parser::ProgramParser;

use crate::SemanticVisitor;

#[test]
pub fn simple_typing() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = 1 in { x + 1 ;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.main_expression.accept(&mut semantic_visitor);

    let dec = &answ
        .main_expression
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    let expression = &answ
        .main_expression
        .as_let_in()
        .unwrap()
        .body
        .as_block()
        .unwrap()
        .body_items[0]
        .as_expression()
        .unwrap();

    let var = expression.as_bin_op().unwrap().lhs.as_variable().unwrap();

    assert_eq!(semantic_visitor.errors.len(), 0);
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::Number)));
    assert_eq!(var.info.ty, Some(Type::BuiltIn(BuiltInType::Number)))
}

#[test]
pub fn binary_op_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = 1 in { x + true ;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.main_expression.accept(&mut semantic_visitor);
    assert_eq!(
        semantic_visitor.errors,
        vec!["Type mismatch: Cannot apply + to operands of type Number and Bool".to_string()]
    );
}

#[test]
pub fn unary_op_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = true in { -x ;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.main_expression.accept(&mut semantic_visitor);

    assert_eq!(
        semantic_visitor.errors,
        vec!["Type mismatch: Cannot apply - to operand of type Bool".to_string()]
    );
}

#[test]
pub fn dassing_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = true in { x:=3 ;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.main_expression.accept(&mut semantic_visitor);

    assert_eq!(
        semantic_visitor.errors,
        vec!["Type mismatch: x is Bool but is being reassigned with Number".to_string()]
    );
}

#[test]
pub fn simple_inference_test() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse("let x = if (true) true else 3 in { x + 1 ;};")
        .unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.main_expression.accept(&mut semantic_visitor);

    let dec = &answ
        .main_expression
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_visitor.errors.len(), 0);
    assert_eq!(dec.info.ty, None)
}

#[test]
pub fn nested_inference() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse("let x = 1 in { let y = 1 > 0 in { if (y == true) x else 0; } ;};")
        .unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    let expr_type = answ.main_expression.accept(&mut semantic_visitor);

    assert_eq!(semantic_visitor.errors.len(), 0);

    let let_in = answ.main_expression.as_let_in().unwrap();

    let dec_id = &let_in.assignment.identifier;

    let inner_let_in = let_in.body.as_block().unwrap().body_items[0]
        .as_expression()
        .unwrap()
        .as_let_in()
        .unwrap();

    let inner_dec_id = &inner_let_in.assignment.identifier;

    assert_eq!(dec_id.info.ty, Some(Type::BuiltIn(BuiltInType::Number)));
    assert_eq!(inner_dec_id.info.ty, Some(Type::BuiltIn(BuiltInType::Bool)));
    assert_eq!(expr_type, Some(Type::BuiltIn(BuiltInType::Number)));
}
