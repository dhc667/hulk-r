use error_handler::error_handler::ErrorHandler;
use generated_parser::ProgramParser;

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
    let program = "type Point {x = 0; y = 0; } type Point {a = 0; b = 0; }";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);
    assert_eq!(semantic_analyzer.type_definitions.is_defined("Point"), true);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Already exists a type or protocol `Point`."]
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
    let program = "constant x: Number = 0; constant x: Number = 1;";
    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);
    assert_eq!(semantic_analyzer.var_definitions.is_defined("x"), true);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Constant `x` is already defined."]
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
    let program =
        "function f(x: Number): Number { return x; } function f(x: Number): Number { return x; }";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Function `f` is already defined."]
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
    let program = "function f(x: Number): Number { return x; } f(true);";
    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec![
            "Semantic Error: Function `f` expects parameter `0` of type `Number`, but got `Boolean`."
        ]
    );
}

#[test]
fn define_global_function_and_use_it_wrong_type2() {
    let program = "function f(x: Number): Number { return x; } f(1, 2);";
    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Function `f` expects 1 parameters, but 2 were provided."]
    );
}

#[test]
fn define_global_function_and_use_it_wrong3() {
    let program = "function f(): Number { return 3; } f(1, 2);";
    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(semantic_analyzer.func_definitions.is_defined("f"), true);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Function `f` expects 0 parameters, but 2 were provided."]
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

#[test]
fn try_dassign_constant() {
    let program = "constant zero: Number = 0; zero:= 2;";
    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(semantic_analyzer.var_definitions.is_defined("zero"), true);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: `zero` is not a valid assignment target.".to_string()]
    )
}

#[test]
fn shadowed_dassignment_to_constant() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            constant zero: Number = 0; 
            let zero = 1 in {
                zero := 3;
            };
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.var_definitions.is_defined("zero"), true);
    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn unknown_annotation_in_global_function_param() {
    let program = "
            function f(x:Number, y: Boniato): Number {x;}
        ";
    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Type or protocol `Boniato` is not defined.".to_string(),]
    );
}

#[test]
fn unknown_annotation_in_constant_definition() {
    let program = "
            constant x: Boniato = 3;
        ";
    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Type or protocol `Boniato` is not defined.".to_string(),]
    );
}

#[test]
fn using_type_arg_in_method() {
    let program = "
        type A(n: Number) {
            get() => n;
        }
    ";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Variable `n` is not defined."]
    );
}

#[test]
fn using_other_param_in_attribute() {
    let program = "
        type A() {
            x = 3;
            y = self;
        }
    ";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Variable `self` is not defined."]
    );
}

#[test]
fn using_other_param_in_attribute2() {
    let program = "
        type A() {
            x = 3;
            y = self.x;
        }
    ";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Variable `self` is not defined."]
    );
}

#[test]
fn member_of_an_unresolved_variable() {
    let program = "
        type A() {
            get() => 3;
        }
        print(a.get());
    ";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Variable `a` is not defined."]
    );
}
