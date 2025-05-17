use parser::ProgramParser;

use crate::semantic_analyzer::SemanticAnalyzer;

#[test]
fn test_define_type() {
    let p = ProgramParser::new();

    let mut answ = p.parse("type Point {x = 0; y = 0; }").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("Point"), true);
    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn test_define_type_twice() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("type Point {x = 0; y = 0; } type Point {a = 0; b = 0; }")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("Point"), true);
    assert_eq!(
        result.err().unwrap(),
        vec!["Type Point is already defined".to_string()]
    );
}

#[test]
fn test_define_several_types() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "type Point {x = 0; y = 0; } 
          type Point2 {a = 0; b = 0; } 
          type Point3 {a = 0; b = 0; } 
          type Point4 {a = 0; b = 0; }",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("Point"), true);
    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("Point2"),
        true
    );

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("Point3"),
        true
    );

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("Point4"),
        true
    );

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn test_define_built_in_types() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "type string {x = \"awa\"; } 
          type bool {a = 0; b = 0; } 
          type number {a = 0; b = 0; } 
          type object {a = 0; b = 0; }",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("string"),
        true
    );
    assert_eq!(semantic_analyzer.type_definitions.is_defined("bool"), true);

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("number"),
        true
    );

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("object"),
        true
    );

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Type string is already defined".to_string(),
            "Type bool is already defined".to_string(),
            "Type number is already defined".to_string(),
            "Type object is already defined".to_string(),
        ]
    );
}
