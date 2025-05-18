use ast::typing::{BuiltInType, Type};
use parser::ProgramParser;

use crate::semantic_analyzer::SemanticAnalyzer;

#[test]
pub fn simple_typing() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = 1 in { x + 1 ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

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

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::Number)));
    assert_eq!(var.info.ty, Some(Type::BuiltIn(BuiltInType::Number)))
}

#[test]
pub fn binary_op_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = 1 in { x + true ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot apply + to operands of type Number and Boolean".to_string()]
    );
}

#[test]
pub fn unary_op_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = true in { -x ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot apply - to operand of type Boolean".to_string()]
    );
}

#[test]
pub fn dassing_error() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = true in { x:=3 ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: x is Boolean but is being reassigned with Number".to_string()]
    );
}

#[test]
pub fn simple_inference_test() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse("let x = if (true) true else 3 in { x + 1 ;};")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot apply + to operands of type Object and Number".to_string()]
    );
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::Object)));
}

#[test]
pub fn nested_inference() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse(
            "
        let result = 
            let x = 1 in { 
                let y = 1 > 0 in { 
                    if (y == true) x else 0; 
                } ;
            } 
            in {result;};",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);

    let outer_let_in = answ.expressions[0].as_let_in().unwrap();

    let expr_type = outer_let_in.assignment.identifier.info.ty.clone();

    let let_in = outer_let_in.assignment.rhs.as_let_in().unwrap();

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

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::String)));
}

#[test]
pub fn list_typing() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = [1, 2, 3] in { x ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(
        dec.info.ty,
        Some(Type::Iterable(Box::new(Type::BuiltIn(BuiltInType::Number))))
    );
}

#[test]
pub fn list_typing_with_different_types() {
    let p = ProgramParser::new();
    let mut answ = p.parse("let x = [1, 2, \"3\"] in { x ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(
        dec.info.ty,
        Some(Type::Iterable(Box::new(Type::BuiltIn(BuiltInType::Object))))
    );
}

#[test]
pub fn list_indexing_typing() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse(
            "
        let result = let x = [1, 2, 3] in x[0] in {
            result;
        };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::Number)));
}

#[test]
pub fn list_indexing_typing_error() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse(
            "
        let result = let x = [1, 2, 3] in x[true] in {
            result;
        };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot use index of type Boolean to access iterable".to_string()]
    );
}

#[test]
pub fn list_indexing_typing_error_2() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse(
            "
        let result = let x = [1, 2, 3] in let y = true in x[true] in {
            result;
        };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot use index of type Boolean to access iterable".to_string()]
    );
}
// TODO: Add more tests for the following cases:
// - Function calls
