use std::vec;

use ast::typing::to_string;
use error_handler::error_handler::ErrorHandler;
use generated_parser::ProgramParser;

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

    assert!(a_def.members.contains_key("method"));

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn type_defintion_and_usage() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse("type A { field = 3; method() => { 3; }; } let a = new A() in {a;};")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    // revisar por que no devuelve anota bien la variable
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    let a_def = semantic_analyzer
        .type_definitions
        .get_value("A")
        .unwrap()
        .as_defined()
        .unwrap();

    let a_type = answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier
        .info
        .ty
        .clone();

    assert_eq!(a_def.name.id, "A".to_string());

    assert!(a_def.members.contains_key("field"));
    assert!(a_def.members.contains_key("method"));
    assert_eq!(to_string(&a_type), "A".to_string());

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn inherited_member_resolve() {
    let program = "
            type A {
                field = 3;
                method() => { 3; }; 
            } 
            type B inherits A { 
                field2 = 4; 
                method2() => { 4; }; 
            } 
            
            let a = new B() in {
                a.field; 
                a.field2; 
                a.method(); 
                a.method2();
            };";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    let a_def = semantic_analyzer
        .type_definitions
        .get_value("A")
        .unwrap()
        .as_defined()
        .unwrap();

    let b_def = semantic_analyzer
        .type_definitions
        .get_value("B")
        .unwrap()
        .as_defined()
        .unwrap();

    assert_eq!(a_def.name.id, "A".to_string());
    assert_eq!(b_def.name.id, "B".to_string());

    assert!(a_def.members.contains_key("field"));
    assert!(a_def.members.contains_key("method"));

    assert!(b_def.members.contains_key("field2"));
    assert!(b_def.members.contains_key("method2"));

    assert_eq!(
        error_handler.get_raw_errors(),
        vec![
            "Semantic Error: Could not find data member `field`.",
            "Semantic Error: Cannot access member `field2` of type `B`. Properties are private, even to inherited types."
        ]
    );
}

#[test]
fn inherited_member_resolve_with_ambiguity() {
    let program = "
            type A {
                field = 3;
                method() => { 3; }; 
            } 
            type B inherits A { 
                field = 4; 
                method() => { 4; }; 
            } 
            
            let a = new B() in {
                a.field; 
                a.method(); 
            };";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    let a_def = semantic_analyzer
        .type_definitions
        .get_value("A")
        .unwrap()
        .as_defined()
        .unwrap();

    let b_def = semantic_analyzer
        .type_definitions
        .get_value("B")
        .unwrap()
        .as_defined()
        .unwrap();

    assert_eq!(a_def.name.id, "A".to_string());
    assert_eq!(b_def.name.id, "B".to_string());

    assert!(a_def.members.contains_key("field"));
    assert!(a_def.members.contains_key("method"));

    assert!(b_def.members.contains_key("field"));
    assert!(b_def.members.contains_key("method"));

    assert_eq!(
        error_handler.get_raw_errors(),
        vec![
            "Semantic Error: Cannot declare field `field` in type `B`, as it overrides parent definition.",
            "Semantic Error: Cannot access member `field` of type `B`. Properties are private, even to inherited types."
        ]
    );
}

#[test]
fn several_inheritance_member_usage() {
    let program = "
            type A {
                field = 3;
                method() => { 3; }; 
            } 
            type B inherits A { 
                field2 = 4; 
                method2() => { 4; }; 
            } 
            type C inherits B { 
                field3 = 5; 
                method3() => { 5; }; 
            } 
            
            let a = new C() in {
                a.field; 
                a.field2; 
                a.field3; 
                a.method(); 
                a.method2(); 
                a.method3();
            };";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    let a_def = semantic_analyzer
        .type_definitions
        .get_value("A")
        .unwrap()
        .as_defined()
        .unwrap();

    let b_def = semantic_analyzer
        .type_definitions
        .get_value("B")
        .unwrap()
        .as_defined()
        .unwrap();

    let c_def = semantic_analyzer
        .type_definitions
        .get_value("C")
        .unwrap()
        .as_defined()
        .unwrap();

    assert_eq!(a_def.name.id, "A".to_string());
    assert_eq!(b_def.name.id, "B".to_string());
    assert_eq!(c_def.name.id, "C".to_string());

    assert!(a_def.members.contains_key("field"));
    assert!(a_def.members.contains_key("method"));

    assert!(b_def.members.contains_key("field2"));
    assert!(b_def.members.contains_key("method2"));

    assert!(c_def.members.contains_key("field3"));
    assert!(c_def.members.contains_key("method3"));

    assert_eq!(
        error_handler.get_raw_errors(),
        vec![
            "Semantic Error: Could not find data member `field`.",
            "Semantic Error: Could not find data member `field2`.",
            "Semantic Error: Cannot access member `field3` of type `C`. Properties are private, even to inherited types."
        ]
    );
}

#[test]
fn inherited_member_in_operation() {
    let program = "
            type A {
                field = 3;
                method() => { 3; }; 
            } 
            type B inherits A { 
                field2 = 4; 
                method2() => { 4; }; 
            } 
            
            let a = new B(), b = a.field + a.method2() in {
                b;
            };";

    let mut error_handler = ErrorHandler::new(program, 0);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    let a_def = semantic_analyzer
        .type_definitions
        .get_value("A")
        .unwrap()
        .as_defined()
        .unwrap();

    let b_def = semantic_analyzer
        .type_definitions
        .get_value("B")
        .unwrap()
        .as_defined()
        .unwrap();

    let b_type = answ.expressions[0]
        .as_let_in()
        .unwrap()
        .body
        .as_let_in()
        .unwrap()
        .assignment
        .identifier
        .info
        .ty
        .clone();

    assert_eq!(to_string(&b_type), "Number".to_string());

    assert_eq!(a_def.name.id, "A".to_string());
    assert_eq!(b_def.name.id, "B".to_string());

    assert!(a_def.members.contains_key("field"));
    assert!(a_def.members.contains_key("method"));

    assert!(b_def.members.contains_key("field2"));
    assert!(b_def.members.contains_key("method2"));

    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Could not find data member `field`.".to_string()]
    );
}

#[test]
fn infered_type_usage() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A {} 
            type B inherits A {} 
            type C inherits A {}

            let b = new B(), c = new C(), a = [b, c] in {
                a;
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    let a_type = answ.expressions[0]
        .as_let_in()
        .unwrap()
        .body
        .as_let_in()
        .unwrap()
        .body
        .as_let_in()
        .unwrap()
        .assignment
        .identifier
        .info
        .ty
        .clone();

    assert_eq!(to_string(&a_type), "A*".to_string());

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn infered_type_usage_with_indefinition() {
    let program = "
            type A {} 
            type B inherits A {} 
            type C inherits A {}

            let b = new B(), c = new C(), a = [f, c] in {
                a;
            };";
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
        vec!["Semantic Error: Variable `f` is not defined.".to_string(),]
    )
}

#[test]
fn accessing_methods_with_arguments() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A { 
                method(x: Number) => { x; }; 
            } 
            type B inherits A { 
                method2(x: Number) => { x; }; 
            } 
            
            let a = new B() in {
                a.method(3); 
                a.method2(4); 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn accessing_methods_with_arguments_and_ambiguity() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A { 
                method(x: Number) => { x; }; 
            } 
            type B inherits A { 
                method(x: Number) => { x; }; 
            } 
            
            let a = new B() in {
                a.method(3); 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn accesing_methods_with_invalid_amount_parameters() {
    let program = "
            type A { 
                method(x: Number) => { x; }; 
            } 
            type B inherits A { 
                method2(x: Number) => { x; }; 
            } 
            
            let a = new B() in {
                a.method(3, 4); 
                a.method2(4, 5); 
            };";

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
        vec![
            "Semantic Error: Function `method` expects 1 parameters, but 2 were provided.",
            "Semantic Error: Function `method2` expects 1 parameters, but 2 were provided."
        ]
    );
}

#[test]
fn accesing_methods_with_invalid_parameter_types() {
    let program = "
            type A { 
                method(x: Number) => { x; }; 
            }
            type B inherits A { 
                method2(x: Number) => { x; }; 
            }
            
            let a = new B() in {
                a.method(true); 
                a.method2(4.0); 
            };";
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
        vec![
            "Semantic Error: Function `method` expects parameter `0` of type `Number`, but got `Boolean`."
        ]
    );
}

#[test]
fn invalid_method_use() {
    let program = "
            type A { 
                method(x: Number) => { x; }; 
            } 
            
            let a = new A() in {
                a.method(3); 
                a.method2(4); 
            };";
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
        vec!["Semantic Error: Could not find method `method2`."]
    );
}

#[test]
fn type_mismatch_when_using_method() {
    let program = "
            type A { 
                method(x: Boolean): Boolean => { x; }; 
            } 
            
            let a = new A() in {
                3 + a.method(true); 
            };";
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
        vec!["Semantic Error: Cannot apply `+` to operands of type `Number` and `Boolean`."]
    );
}

#[test]
fn declaration_of_type_with_arguments() {
    let p = ProgramParser::new();

    let mut answ = p.parse("type A(x: Number) { field = x; }").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn declaration_of_type_with_arguments_and_usage() {
    let program = "
            type A(x: Number) { field = x; } 
            
            let a = new A(3) in {
                a.field; 
            };";
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
        vec![
            "Semantic Error: Cannot access member `field` of type `A`. Properties are private, even to inherited types."
        ]
    );
}

#[test]
fn declaration_of_type_with_arguments_and_usage2() {
    let program = "
            type A(x: Number) { field = x; } 
            
            let a = new A(3) in {
                true && a.field; 
            };";
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
        vec![
            "Semantic Error: Cannot apply `&&` to operands of type `Boolean` and `Number`.",
            "Semantic Error: Cannot access member `field` of type `A`. Properties are private, even to inherited types.",
        ]
    );
}

#[test]
fn declaration_of_type_with_reference_to_self() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A(x: Number) { 
                field = x; 
                method() => { self.field; }; 
            } 
            
            let a = new A(3) in {
                a.method(); 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn declaration_of_type_with_reference_to_self_wrong() {
    let program = "
            type A(x: Boolean) { 
                field = x; 
                method() => { self.field + 3; }; 
            } 
            
            let a = new A(true) in {
                a.method(); 
            };";
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
        vec!["Semantic Error: Cannot apply `+` to operands of type `Boolean` and `Number`."]
    );
}

#[test]
fn declaration_of_type_with_self_inherited_access() {
    let program = "
            type A { 
                field = 3; 
            } 
            type B inherits A  {
                method() => { self.field; }; 
            } 
            
            let a = new B() in {
                a.method();  
            };";
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
        vec!["Semantic Error: Could not find data member `field`.".to_string()]
    );
}

#[test]
fn declaration_of_type_with_self_inherited_access_wrong() {
    let program = "
            type A { 
                field = 3; 
            } 
            type B inherits A  {
                method() => { self.field + 3; }; 
            } 
            
            let a = new B() in {
                a.method() && true;  
            };";
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
        vec![
            "Semantic Error: Could not find data member `field`.",
            "Semantic Error: Cannot apply `&&` to operands of type `Number` and `Boolean`."
        ]
    );
}

#[test]
fn super_constructor_call() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A(x: Number) { 
                field = x; 
            } 
            type B(x: Number) inherits A(2*x)  { 
            } 
            
            let a = new B(3) in {
                a;
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn super_constructor_call2() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A(x: Number, y: Number, z: Number) { 
                field = x; 
            } 
            type B(x: Number) inherits A(2*x, 3*x, 4*x)  { 
            } 
            
            let a = new B(3) in {
                a;  
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn super_constructor_with_wrong_arguments() {
    let program = "
            type A(x: Boolean) { 
                field = x; 
            } 
            type B(x: Number) inherits A(2*x)  { 
            } 
            
            let a = new B(2) in {
                a;  
            };";

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
        vec!["Semantic Error: Type `A` expects parameter `0` of type `Boolean`, but got `Number`."]
    );
}

#[test]
fn nested_super_constructor_call() {
    let program = "
            type A(x: Number) { 
                field = x; 
            } 
            type B(x: Number) inherits A(2*x)  { 
                field2 = 3; 
            } 
            type C(x: Number) inherits B(2*x)  { 
                field3 = 4; 
            } 
            
            let a = new C(3) in {
                a.field;  
                a.field2;
                a.field3;
            };";

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
        vec![
            "Semantic Error: Could not find data member `field`.".to_string(),
            "Semantic Error: Could not find data member `field2`.".to_string(),
            "Semantic Error: Cannot access member `field3` of type `C`. Properties are private, even to inherited types.".to_string()
        ]
    );
}

#[test]
fn dassigning_to_self() {
    let program = "
            type A { 
                method() => { self := 5; };
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
        vec!["Semantic Error: `self` is not a valid assignment target.".to_string()]
    );
}

#[test]
fn shadowed_dassignment_to_self() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A { 
                method() => { let self = 5 in { self := 3; }; };
            }

            let self = 3 in {
                self := 4;
            };
            ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn list_field() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A { 
                field: Number* = [1, 2, 3];
                getField() => { self.field; }; 
            } 
            
            let a = new A() in {
                a.getField();  
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn mutate_field_in_list() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A {
                array: Number*  = [1];
                getArray() => { self.array; };
            }
            let a: A* = [new A(), new A()] in {
                a[0].getArray()[0] + 1;
            };
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn mutate_field_in_list_incorrect_typing() {
    let program = "
            type A {
                array: Number  = [1];
                getArray() => { self.array; };
            }
            let a: A* = [new A(), new A()] in {
                a[0].getArray()[0] + 1;
            };
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
        vec!["Semantic Error: Cannot assign `Number*` to `Number`.".to_string()]
    );
}

#[test]
fn operation_on_element_without_indexing() {
    let program = "
            type A {
                array: Number*  = [1];
                getArray() => { self.array; };
            }
            let a: A* = [new A(), new A()] in {
                a[0].getArray() + 1;
            };
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
        vec![
            "Semantic Error: Cannot apply `+` to operands of type `Number*` and `Number`."
                .to_string()
        ]
    );
}

#[test]
fn unknown_annotation() {
    let program = "
            let a: Boniato = 1 in {a;};
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
        vec!["Semantic Error: Type or protocol `Boniato` is not defined.".to_string()]
    );
}

#[test]
fn unknown_annotation_in_method_param() {
    let program = "
            type A {
                method(x: Number, y: Boniato, z: Malanga): Number {
                    x;
                }
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
        vec![
            "Semantic Error: Type or protocol `Boniato` is not defined.",
            "Semantic Error: Type of `y` could not be resolved",
            "Semantic Error: Type or protocol `Malanga` is not defined.",
            "Semantic Error: Type of `z` could not be resolved"
        ]
    );
}

#[test]
fn unknown_annotation_in_method_return() {
    let program = "
            type A {
                method(x: Number, y: Number, z: Number): Boniato {
                    x;
                }
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
        vec!["Semantic Error: Type or protocol `Boniato` is not defined.".to_string(),]
    );
}

#[test]
fn unknown_annotation_in_type_arg() {
    let program = "
            type A(x: Boniato, y: Number){}
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
        vec![
            "Semantic Error: Type or protocol `Boniato` is not defined.",
            "Semantic Error: Type of `x` could not be resolved"
        ]
    );
}

#[test]
fn reassign_nonexisting_property() {
    let program = "
            type A {
                x=3;
            }
            let a = new A() in {
                a.y := 1;
            };
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
        vec!["Semantic Error: Could not find data member `y`."]
    );
}

#[test]
fn use_field_without_self() {
    let program = "
            type A {
                x=3;
                method(): Number {
                    x;
                }
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
        vec!["Semantic Error: Variable `x` is not defined.".to_string()]
    );
}

#[test]
fn field_with_same_name_as_param() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A(x: Number) {
                x= x;
                method(x: Number): Number {
                    self.x + x;
                }
            }
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn field_override_errors() {
    let program = "
            type A {
                x = 1;
                method():Number => 1;
            }
            type B inherits A {
                x = 2;
                method():Number => 2;
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
        vec![
            "Semantic Error: Cannot declare field `x` in type `B`, as it overrides parent definition."
        ]
    );
}

#[test]
fn method_incorrect_override() {
    let program = "
            type A {
                foo():Number => 1;
            }
            type B inherits A {
                foo():Boolean => true;
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
        vec![
            "Semantic Error: Method `foo` in type `B`, does not properly overrides parent definition."
        ]
    );
}

#[test]
fn method_incorrect_override_diferent_amount() {
    let program = "
            type A {
                foo():Number => 1;
            }
            type B inherits A {
                foo(x: Number):Number => x;
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
        vec![
            "Semantic Error: Method `foo` in type `B`, does not properly overrides parent definition."
        ]
    );
}

#[test]
fn method_override_variant_1() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type O {}
            type N inherits O {}

            type A {
                foo(x: N):N => x;
            }
            type B inherits A {
                foo(x: O):N => new N();
            }
            ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn method_override_variant_2() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type O {}
            type N inherits O {}

            type A {
                foo(x: N):O => new N();
            }
            type B inherits A {
                foo(x: N):N => x;
            }
            ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn method_override_variant_3() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type O {}
            type N inherits O {}

            type A {
                foo(x: N):O => x;
            }
            type B inherits A {
                foo(x: O):N => new N();
            }
            ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn field_override_should_fail() {
    let program = r#"
        type A { x = 1; }
        type B inherits A { x = 2; }
    "#;
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
        vec![
            "Semantic Error: Cannot declare field `x` in type `B`, as it overrides parent definition."
        ]
    );
}

#[test]
fn method_override_wrong_arg_count() {
    let program = "
        type O {}
        type N inherits O {}

        type A { 
            foo(x: N): N => x; 
        }
        type B inherits A { 
            foo(x: N, y: N): N => x; 
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
        vec![
            "Semantic Error: Method `foo` in type `B`, does not properly overrides parent definition."
        ]
    );
}

#[test]
fn method_override_wrong_return_covariance() {
    let program = r#"
        type O {}
        type N inherits O {}

        type A { 
            foo(x: N): O => x; 
        }
        type B inherits A { 
            foo(x: N): N => x; 
        }
        type C inherits B { 
            foo(x: N): B => new B(); 
        }
    "#;

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
        vec![
            "Semantic Error: Method `foo` in type `C`, does not properly overrides parent definition."
        ]
    );
}

#[test]
fn complicated_inheritance_chain() {
    let program = "
        type O {}
        type N inherits O {}

        type A { foo(x: O): O => x; }
        type B inherits A { foo(x: N): O => x; }
        type C inherits B { foo(x: N): N => x; }
        type D inherits C { foo(x: N): N => x; }
        type E inherits D { foo(x: N): N => x; }
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
        vec![
            "Semantic Error: Method `foo` in type `B`, does not properly overrides parent definition."
        ]
    )
}

#[test]
fn complicated_inheritance_cycle_should_fail() {
    let p = ProgramParser::new();
    let mut answ = p
        .parse(
            r#"
        type O {}
        type N inherits O {}

        type A inherits C { foo(x: N): O => x; }
        type B inherits A { foo(x: O): O => x; }
        type C inherits B { foo(x: O): N => x; }
    "#,
        )
        .unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);
    assert!(result.is_err(), "Should detect inheritance cycle");
}

#[test]
fn override_from_far_ancestor() {
    let program = "
            type O {}
            type N inherits O {}

            type A {
                foo(x: N):O => x;
                x = 1;
            }
            type C inherits A {}
            type B inherits C {
                foo(x: O):N => new N();
                x = 2;
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
        vec![
            "Semantic Error: Cannot declare field `x` in type `B`, as it overrides parent definition."
                .to_string(),
        ]
    );
}
