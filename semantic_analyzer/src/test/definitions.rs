use crate::SemanticVisitor;
use ast::VisitableExpression;
use parser::ProgramParser;

#[test]
fn not_defined_variable() {
    let p = ProgramParser::new();

    let mut answ = p.parse("x + 2;").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.main_expression.accept(&mut semantic_visitor);
    assert_eq!(
        semantic_visitor.errors,
        vec!["Variable x is not defined".to_string()]
    );
}

#[test]
fn shadow_different_let_in() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("let x = 1 + 2 in let x = x + 2 in {x + 2;};")
        .unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.main_expression.accept(&mut semantic_visitor);
}

#[test]
fn shadow_in_same_let_in() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x = 1 + 2, x = x + 2 in {x + 2;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.main_expression.accept(&mut semantic_visitor);
}

#[test]
fn lookup_in_let_in() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x = 1 + 2 in let y = 4 in {x + 2;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.main_expression.accept(&mut semantic_visitor);
}

#[test]
fn lookup_in_let_in_with_shadow() {
    let p = ProgramParser::new();

    let mut answ = p.parse("{ let x = 1 + 2 in let x = 4 in {x + 2;}; };").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.main_expression.accept(&mut semantic_visitor);
}

#[test]
fn not_defined_variable_different_let_in() {
    let p = ProgramParser::new();

    let mut answ = p.parse("{ let x=3 in {x;}; x+18; };").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.main_expression.accept(&mut semantic_visitor);

    assert_eq!(
        semantic_visitor.errors,
        vec!["Variable x is not defined".to_string()]
    );
}

#[test]
fn several_undefinitions() {
    let p = ProgramParser::new();

    let mut answ = p.parse("{ let x=3, y=4, z=5 in {x;}; x+y+z+18; };").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.main_expression.accept(&mut semantic_visitor);

    assert_eq!(
        semantic_visitor.errors,
        vec![
            "Variable x is not defined".to_string(),
            "Variable y is not defined".to_string(),
            "Variable z is not defined".to_string()
        ]
    );
}
