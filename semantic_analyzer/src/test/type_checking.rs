use ast::Visitable;
use parser::ProgramParser;

use crate::{BuiltInType, SemanticVisitor, Type, TypeCheckerVisitor};

#[test]
pub fn simple_typing() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = 1 in { x + 1 ;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.accept(&mut semantic_visitor);

    let mut type_checker_visitor = TypeCheckerVisitor::new(semantic_visitor.definitions);

    answ.accept(&mut type_checker_visitor);

    assert_eq!(type_checker_visitor.errors.len(), 0);
    assert_eq!(
        type_checker_visitor.def_context.get_type("x", 1),
        Some(Type::BuiltIn(BuiltInType::Number))
    )
}

#[test]
pub fn binary_op_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = 1 in { x + true ;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.accept(&mut semantic_visitor);

    let mut type_checker_visitor = TypeCheckerVisitor::new(semantic_visitor.definitions);

    answ.accept(&mut type_checker_visitor);

    assert_eq!(
        type_checker_visitor.errors,
        vec!["Type mismatch: Cannot apply + to operands of type number and bool".to_string()]
    );
}

#[test]
pub fn unary_op_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = true in { -x ;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.accept(&mut semantic_visitor);

    let mut type_checker_visitor = TypeCheckerVisitor::new(semantic_visitor.definitions);

    answ.accept(&mut type_checker_visitor);

    assert_eq!(
        type_checker_visitor.errors,
        vec!["Type mismatch: Cannot apply - to operand of type bool".to_string()]
    );
}

#[test]
pub fn dassing_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = true in { x:=3 ;};").unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.accept(&mut semantic_visitor);

    let mut type_checker_visitor = TypeCheckerVisitor::new(semantic_visitor.definitions);

    answ.accept(&mut type_checker_visitor);

    assert_eq!(
        type_checker_visitor.errors,
        vec!["Type mismatch: x is bool but is being reassigned with number".to_string()]
    );
}

#[test]
pub fn simple_inference_test() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse("let x = if (true) true else 3 in { x + 1 ;};")
        .unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.accept(&mut semantic_visitor);

    let mut type_checker_visitor = TypeCheckerVisitor::new(semantic_visitor.definitions);

    answ.accept(&mut type_checker_visitor);

    assert_eq!(type_checker_visitor.errors.len(), 0);
    assert_eq!(type_checker_visitor.def_context.get_type("x", 1), None)
}

#[test]
pub fn nested_inference() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse("let x = 1 in { let y = 1 > 0 in { if (y == true) x else 0; } ;};")
        .unwrap();

    let mut semantic_visitor = SemanticVisitor::new();
    answ.accept(&mut semantic_visitor);

    let mut type_checker_visitor = TypeCheckerVisitor::new(semantic_visitor.definitions);

    let expr_type = answ.accept(&mut type_checker_visitor);

    assert_eq!(type_checker_visitor.errors.len(), 0);
    assert_eq!(
        type_checker_visitor.def_context.get_type("x", 1),
        Some(Type::BuiltIn(BuiltInType::Number))
    );
    assert_eq!(
        type_checker_visitor.def_context.get_type("y", 2),
        Some(Type::BuiltIn(BuiltInType::Bool))
    );
    assert_eq!(expr_type, Some(Type::BuiltIn(BuiltInType::Number)));
}
