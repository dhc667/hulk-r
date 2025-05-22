use parser::ProgramParser;

use crate::semantic_analyzer::SemanticAnalyzer;

#[test]
fn simple_member_definition() {
    let p = ProgramParser::new();

    let mut answ = p.parse("type A { field = 3;}").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    let a_def = semantic_analyzer
        .type_definitions
        .get_value("A")
        .unwrap()
        .as_defined()
        .unwrap();

    assert_eq!(a_def.name.id, "A".to_string());

    assert!(a_def.members.contains_key("field"));

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn simple_method_definition() {
    let p = ProgramParser::new();

    let mut answ = p.parse("type A { method() => { 3; };}").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    let a_def = semantic_analyzer
        .type_definitions
        .get_value("A")
        .unwrap()
        .as_defined()
        .unwrap();

    assert_eq!(a_def.name.id, "A".to_string());

    assert!(a_def.methods.contains_key("method"));

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}
