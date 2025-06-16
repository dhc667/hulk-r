use crate::semantic_analyzer::SemanticAnalyzer;
use error_handler::error_handler::ErrorHandler;
use generated_parser::ProgramParser;

fn analyze_and_get_errors(program: &str) -> Vec<String> {
    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);
    error_handler.get_raw_errors()
}

#[test]
fn not_defined_variable() {
    let program = "x + 2;";
    assert_eq!(
        analyze_and_get_errors(program),
        vec!["Semantic Error: Variable `x` is not defined.".to_string()]
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
    let program = "{ let x=3 in {x;}; x+18; };";
    assert_eq!(
        analyze_and_get_errors(program),
        vec!["Semantic Error: Variable `x` is not defined.".to_string()]
    );
}

#[test]
fn several_undefinitions() {
    let program = "{ let x=3, y=4, z=5 in {x;}; x+y+z+18; };";
    assert_eq!(
        analyze_and_get_errors(program),
        vec![
            "Semantic Error: Variable `x` is not defined.",
            "Semantic Error: Variable `y` is not defined.",
            "Semantic Error: Variable `z` is not defined."
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
    let program = "let x: Number = true in {x;};";
    assert_eq!(
        analyze_and_get_errors(program),
        vec!["Semantic Error: Cannot assign `Boolean` to `Number`.".to_string()]
    );
}

#[test]
fn list_definition() {
    let p = ProgramParser::new();

    let mut answ = p.parse("let x: Boolean* = [true, false] in x;").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    // No errors expected
    assert!(semantic_analyzer.errors.is_empty());
}

#[test]
fn mutate_field() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A {
                x  = 3;
                method(): Number {
                    self.x := 4;  
                }
            }
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    // No errors expected
    assert!(semantic_analyzer.errors.is_empty());
}

#[test]
fn mutate_indexing() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            let a = [1, 2, 3] in {
                a[0] := 4;
                a[1] := 5;
                a[2] := 6;
            };
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    // No errors expected
    assert!(semantic_analyzer.errors.is_empty());
}

#[test]
fn mutate_non_variable() {
    let program = "3:= 4;";
    assert_eq!(
        analyze_and_get_errors(program),
        vec!["Semantic Error: Only variables and self properties can be assigned.".to_string()]
    );
}

#[test]
fn mutate_field_outside_definition() {
    let program = "
            type A {
                x  = 3;
            }
            let a = new A() in {
                a.x := 4;
            };
        ";
    assert_eq!(
        analyze_and_get_errors(program),
        vec![
            "Semantic Error: Cannot access member `x` of type `A`. Properties are private, even to inherited types."
                .to_string()
        ]
    );
}

#[test]
pub fn print_defined() {
    let p = ProgramParser::new();

    let mut answ = p.parse("print(1);").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    // No errors expected
    assert!(semantic_analyzer.errors.is_empty());
}

#[test]
pub fn constant_definition() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
        constant EPSILON: Number = 0.00000001;
        constant PI: Number = 3.141592653589793;

        function foo(): Number => EPSILON;

        print(foo());
    ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    // No errors expected
    assert!(semantic_analyzer.errors.is_empty());
}
