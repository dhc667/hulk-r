use crate::semantic_analyzer::SemanticAnalyzer;
use parser::ProgramParser;

#[test]
fn not_defined_variable() {
    let p = ProgramParser::new();

    let mut answ = p.parse("x + 2;").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Variable x is not defined".to_string()]
    );
}

#[test]
fn shadow_different_let_in() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("let x = 1 + 2 in let x = x + 2 in {x + 2;};")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    // No errors expected
    assert!(semantic_analyzer.errors.is_empty());
}

#[test]
fn shadow_in_same_let_in() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x = 1 + 2, x = x + 2 in {x + 2;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    // No errors expected
    assert!(semantic_analyzer.errors.is_empty());
}

#[test]
fn lookup_in_let_in() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x = 1 + 2 in let y = 4 in {x + 2;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    // No errors expected
    assert!(semantic_analyzer.errors.is_empty());
}

#[test]
fn lookup_in_let_in_with_shadow() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("{ let x = 1 + 2 in let x = 4 in {x + 2;}; };")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    // No errors expected
    assert!(semantic_analyzer.errors.is_empty());
}

#[test]
fn not_defined_variable_different_let_in() {
    let p = ProgramParser::new();

    let mut answ = p.parse("{ let x=3 in {x;}; x+18; };").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Variable x is not defined".to_string()]
    );
}

#[test]
fn several_undefinitions() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("{ let x=3, y=4, z=5 in {x;}; x+y+z+18; };")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec![
            "Variable x is not defined".to_string(),
            "Variable y is not defined".to_string(),
            "Variable z is not defined".to_string()
        ]
    );
}

#[test]
fn func_definitions() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("function foo(x: Number, y: Number): Number { x+y;}")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());

    assert!(semantic_analyzer.func_definitions.is_defined("foo"));
    assert!(semantic_analyzer.var_definitions.is_defined("$fooInstance"));
    assert!(
        semantic_analyzer
            .type_definitions
            .is_defined("$fooTypeWrapper")
    );
    assert!(
        semantic_analyzer
            .type_definitions
            .get_value("$fooTypeWrapper")
            .unwrap()
            .as_defined()
            .unwrap()
            .members
            .contains_key("invoke")
    );

    assert_eq!(
        semantic_analyzer
            .var_definitions
            .get_value("$fooInstance")
            .unwrap()
            .ty
            .clone()
            .unwrap()
            .as_defined()
            .unwrap()
            .id,
        "$fooTypeWrapper"
    );
}

#[test]
fn anotated_var_with_wrong_value_type() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x: Number = true in {x;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot assign Boolean to Number".to_string()]
    );
}
