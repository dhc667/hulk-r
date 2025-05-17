use crate::{DefinitionInfo, SemanticVisitor};
use ast::VisitableExpression;
use generator::context::Context;
use parser::ProgramParser;

#[test]
fn not_defined_variable() {
    let p = ProgramParser::new();

    let mut answ = p.parse("x + 2;").unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);

    answ.expressions[0].accept(&mut semantic_visitor);
    assert_eq!(errors, vec!["Variable x is not defined".to_string()]);
}

#[test]
fn shadow_different_let_in() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("let x = 1 + 2 in let x = x + 2 in {x + 2;};")
        .unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);

    answ.expressions[0].accept(&mut semantic_visitor);
}

#[test]
fn shadow_in_same_let_in() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x = 1 + 2, x = x + 2 in {x + 2;};").unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);

    answ.expressions[0].accept(&mut semantic_visitor);
}

#[test]
fn lookup_in_let_in() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x = 1 + 2 in let y = 4 in {x + 2;};").unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);

    answ.expressions[0].accept(&mut semantic_visitor);
}

#[test]
fn lookup_in_let_in_with_shadow() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("{ let x = 1 + 2 in let x = 4 in {x + 2;}; };")
        .unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);

    answ.expressions[0].accept(&mut semantic_visitor);
}

#[test]
fn not_defined_variable_different_let_in() {
    let p = ProgramParser::new();

    let mut answ = p.parse("{ let x=3 in {x;}; x+18; };").unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);

    answ.expressions[0].accept(&mut semantic_visitor);

    assert_eq!(errors, vec!["Variable x is not defined".to_string()]);
}

#[test]
fn several_undefinitions() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("{ let x=3, y=4, z=5 in {x;}; x+y+z+18; };")
        .unwrap();

    let mut definitions: Context<DefinitionInfo> = Context::new_one_frame();
    let mut errors: Vec<String> = Vec::new();

    let mut semantic_visitor = SemanticVisitor::new(&mut definitions, &mut errors);

    answ.expressions[0].accept(&mut semantic_visitor);

    assert_eq!(
        errors,
        vec![
            "Variable x is not defined".to_string(),
            "Variable y is not defined".to_string(),
            "Variable z is not defined".to_string()
        ]
    );
}
