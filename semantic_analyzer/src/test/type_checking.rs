use ast::{
    VisitableExpression,
    typing::{BuiltInType, Type},
};
use generator::context::Context;
use parser::ProgramParser;

use crate::{DefinitionInfo, SemanticVisitor};

#[test]
pub fn simple_typing() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = 1 in { x + 1 ;};").unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);
    answ.expressions[0].accept(&mut semantic_visitor);

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    let expression = &answ.expressions[0]
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

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);
    answ.expressions[0].accept(&mut semantic_visitor);
    assert_eq!(
        errors,
        vec!["Type mismatch: Cannot apply + to operands of type Number and Boolean".to_string()]
    );
}

#[test]
pub fn unary_op_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = true in { -x ;};").unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);
    answ.expressions[0].accept(&mut semantic_visitor);

    assert_eq!(
        errors,
        vec!["Type mismatch: Cannot apply - to operand of type Boolean".to_string()]
    );
}

#[test]
pub fn dassing_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = true in { x:=3 ;};").unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);
    answ.expressions[0].accept(&mut semantic_visitor);

    assert_eq!(
        errors,
        vec!["Type mismatch: x is Boolean but is being reassigned with Number".to_string()]
    );
}

#[test]
pub fn simple_inference_test() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse("let x = if (true) true else 3 in { x + 1 ;};")
        .unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);
    answ.expressions[0].accept(&mut semantic_visitor);

    let dec = &answ.expressions[0]
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

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);
    let expr_type = answ.expressions[0].accept(&mut semantic_visitor);

    assert_eq!(semantic_visitor.errors.len(), 0);

    let let_in = answ.expressions[0].as_let_in().unwrap();

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

#[test]
pub fn string_typing() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = \"boniato\" in { x ;};").unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);
    answ.expressions[0].accept(&mut semantic_visitor);

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_visitor.errors.len(), 0);
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::String)));
}

// TODO: Add more tests for the following cases:
// - Function calls
// - List indexing
// - List literals
