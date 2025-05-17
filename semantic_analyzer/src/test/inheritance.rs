use ast::typing::{BuiltInType, Type};
use parser::ProgramParser;

use crate::semantic_analyzer::SemanticAnalyzer;

#[test]
fn built_ins_inheritance() {
    let p = ProgramParser::new();

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
    let p = ProgramParser::new();

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
    let p = ProgramParser::new();

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
    let p = ProgramParser::new();

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
