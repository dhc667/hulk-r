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
fn define_global_variable() {
    let p = ProgramParser::new();

    let mut answ = p.parse("constant x: Number = 0;").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.var_definitions.is_defined("x"), true);
    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn define_global_variable_twice() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("constant x: Number = 0; constant x: Number = 1;")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(semantic_analyzer.var_definitions.is_defined("x"), true);
    assert_eq!(
        semantic_analyzer.errors,
        vec!["Constant x is already defined".to_string()]
    );
}

#[test]
fn define_global_function() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("function f(x: Number): Number { return x; }")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn define_global_function_twice() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("function f(x: Number): Number { return x; } function f(x: Number): Number { return x; }")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert_eq!(
        result.err().unwrap(),
        vec!["Function f is already defined".to_string()]
    );
}

#[test]
fn define_global_function_with_same_name_as_type() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("type f {x = 0; y = 0; } function f(x: Number): Number { return x; }")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn define_global_function_with_same_name_as_variable() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("constant f: Number = 0; function f(x: Number): Number { return x; }")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn define_global_function_and_use_it() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("function f(x: Number): Number { return x; } function g(x: Number): Number { return f(x); }")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert_eq!(semantic_analyzer.func_definitions.is_defined("g"), true);
    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn define_global_function_and_check_type() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("function f(x: Number): Number { return x; } 2 + f(2);")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn define_global_function_and_use_it_wrong_type() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("function f(x: Number): Number { return x; } f(true);")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert_eq!(
        result.err().unwrap(),
        vec!["Function f expects parameter 0 of type Number, but got Boolean".to_string()]
    );
}

#[test]
fn define_global_function_and_use_it_wrong_type2() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("function f(x: Number): Number { return x; } f(1, 2);")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert_eq!(
        result.err().unwrap(),
        vec!["Function f expects 1 parameters, but 2 were provided".to_string()]
    );
}

#[test]
fn define_global_function_and_use_it_wrong3() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("function f(): Number { return 3; } f(1, 2);")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert_eq!(
        result.err().unwrap(),
        vec!["Function f expects 0 parameters, but 2 were provided".to_string()]
    );
}

#[test]
fn define_global_arrow_function() {
    let p = ProgramParser::new();

    let mut answ = p.parse("function f(x: Number): Number => x;").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert!(result.is_ok(), "Errors: {:?}", result.err());
}
