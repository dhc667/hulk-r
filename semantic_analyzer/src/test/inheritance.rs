use ast::typing::{BuiltInType, Type};
use error_handler::error_handler::ErrorHandler;
use parser::parser::Parser;

use crate::semantic_analyzer::SemanticAnalyzer;

#[test]
fn built_ins_inheritance() {
    let p = Parser::new();

    let mut answ = p.parse("3;").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("String"),
        true
    );
    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("Boolean"),
        true
    );

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("Number"),
        true
    );

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("Object"),
        true
    );

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("String")
            .unwrap()
            .clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("Boolean")
            .unwrap()
            .clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("Number")
            .unwrap()
            .clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("Object")
            .unwrap()
            .clone(),
        None,
    );

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn basic_inheritance() {
    let p = Parser::new();

    let mut answ = p
        .parse(
            "type A {} 
        type B inherits A {}",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("A"), true);

    assert_eq!(
        semantic_analyzer.type_hierarchy.get("A").unwrap().clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("B"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("B")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "A".to_string(),
    );

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn before_delcared() {
    let p = Parser::new();

    let mut answ = p
        .parse(
            "
          type B inherits A {}
          type A {} 
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("A"), true);

    assert_eq!(
        semantic_analyzer.type_hierarchy.get("A").unwrap().clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("B"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("B")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "A".to_string(),
    );

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn nested_inheritance() {
    let p = Parser::new();

    let mut answ = p
        .parse(
            "
          type B inherits A {}
          type A inherits C {}
          type C {} 
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("A"), true);

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("A")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "C".to_string(),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("B"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("B")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "A".to_string(),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("C"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("C")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "Object".to_string(),
    );

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn inheritance_cycle() {
    let p = Parser::new();

    let mut answ = p
        .parse(
            "
          type B inherits A {}
          type A inherits B {} 
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("A"), true);

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("A")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "B".to_string(),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("B"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("B")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "A".to_string(),
    );

    println!("{:?}", semantic_analyzer.errors);

    assert!(result.is_err(), "Errors: {:?}", result.err());
}

#[test]
fn complicated_inheritance_cycle() {
    let p = Parser::new();

    let mut answ = p
        .parse(
            "
          type A inherits B {}
          type B inherits C {}
          type C inherits D {}
          type D inherits E {}
          type E inherits F {}
          type F inherits G {}
          type G inherits H {}
          type H inherits I {}
          type I inherits J {}
          type J inherits K {}
          type K inherits L {}
          type L inherits M {}
          type M inherits N {}
          type N inherits A {}
          type P inherits Q {}
          type W inherits O {}
          type O inherits Y {}
          type Q inherits W {}
          type Z inherits W {}
          type Y { message = \"boniato\";}
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    println!("{:?}", semantic_analyzer.errors);

    assert!(result.is_err(), "Errors: {:?}", result.err());
}

#[test]
fn redeclare_object() {
    let program = "
        type Object {}";

    let mut error_handler = ErrorHandler::new(program);
    let p = Parser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Already exists a type or protocol `Object`."]
    )
}
