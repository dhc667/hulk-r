use crate::{
    grammar::ExpressionListParser,
    visitors::{implementations::semantic_visitor::SemanticVisitor, visitable::Visitable},
};

#[test]
#[should_panic(expected = "Variable x is not defined")]
fn not_defined_variable() {
    let p = ExpressionListParser::new();

    let mut answ = p.parse("x + 2;").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.accept(&mut semantic_visitor);
}

#[test]
fn shadow_different_let_in() {
    let p = ExpressionListParser::new();

    let mut answ = p
        .parse("let x = 1 + 2 in let x = x + 2 in {x + 2;};")
        .unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.accept(&mut semantic_visitor);
}

#[test]
fn shadow_in_same_let_in() {
    let p = ExpressionListParser::new();

    let mut answ = p.parse("let x = 1 + 2, x = x + 2 in {x + 2;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.accept(&mut semantic_visitor);
}

#[test]
fn lookup_in_let_in() {
    let p = ExpressionListParser::new();

    let mut answ = p.parse("let x = 1 + 2 in let y = 4 in {x + 2;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.accept(&mut semantic_visitor);
}

#[test]
fn lookup_in_let_in_with_shadow() {
    let p = ExpressionListParser::new();

    let mut answ = p.parse("let x = 1 + 2 in let x = 4 in {x + 2;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.accept(&mut semantic_visitor);
}

#[test]
#[should_panic(expected = "Variable x is not defined")]
fn not_defined_variable_different_let_in() {
    let p = ExpressionListParser::new();

    let mut answ = p.parse("let x=3 in {x;}; x+18;").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();

    answ.accept(&mut semantic_visitor);
}
