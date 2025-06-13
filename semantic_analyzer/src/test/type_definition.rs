use std::vec;

use ast::typing::to_string;
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
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
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
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

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
        result.err().unwrap(),
        vec![
            "Could not find data member field".to_string(),
            "Cannot access member field2 of type B. Properties are private, even to inherited types.".to_string()
        ]
    );
}

#[test]
fn inherited_member_resolve_with_ambiguity() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
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
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

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
        result.err().unwrap(),
        vec![
            "Cannot access member field of type B. Properties are private, even to inherited types.".to_string()
        ]
    );
}

#[test]
fn several_inheritance_member_usage() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
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
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

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
        result.err().unwrap(),
        vec![
            "Could not find data member field".to_string(),
            "Could not find data member field2".to_string(),
            "Cannot access member field3 of type C. Properties are private, even to inherited types.".to_string()
        ]
    );
}

#[test]
fn inherited_member_in_operation() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
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
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

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
        result.err().unwrap(),
        vec!["Could not find data member field".to_string()]
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
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A {} 
            type B inherits A {} 
            type C inherits A {}

            let b = new B(), c = new C(), a = [f, c] in {
                a;
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Variable f is not defined".to_string(),]
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
                a.method(3, 4); 
                a.method2(4, 5); 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Function method expects 1 parameters, but 2 were provided".to_string(),
            "Function method2 expects 1 parameters, but 2 were provided".to_string()
        ]
    );
}

#[test]
fn accesing_methods_with_invalid_parameter_types() {
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
                a.method(true); 
                a.method2(4.0); 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Function method expects parameter 0 of type Number, but got Boolean".to_string()]
    );
}

#[test]
fn invalid_method_use() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A { 
                method(x: Number) => { x; }; 
            } 
            
            let a = new A() in {
                a.method(3); 
                a.method2(4); 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Could not find method method2".to_string()]
    );
}

#[test]
fn type_mismatch_when_using_method() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A { 
                method(x: Boolean): Boolean => { x; }; 
            } 
            
            let a = new A() in {
                3 + a.method(true); 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Type mismatch: Cannot apply + to operands of type Number and Boolean".to_string()]
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
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A(x: Number) { field = x; } 
            
            let a = new A(3) in {
                a.field; 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Cannot access member field of type A. Properties are private, even to inherited types."
        ]
    );
}

#[test]
fn declaration_of_type_with_arguments_and_usage2() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A(x: Number) { field = x; } 
            
            let a = new A(3) in {
                true && a.field; 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Cannot access member field of type A. Properties are private, even to inherited types.".to_string(),
            "Type mismatch: Cannot apply && to operands of type Boolean and Number".to_string()
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
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A(x: Boolean) { 
                field = x; 
                method() => { self.field + 3; }; 
            } 
            
            let a = new A(true) in {
                a.method(); 
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Type mismatch: Cannot apply + to operands of type Boolean and Number".to_string()]
    );
}

#[test]
fn declaration_of_type_with_self_inherited_access() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A { 
                field = 3; 
            } 
            type B inherits A  {
                method() => { self.field; }; 
            } 
            
            let a = new B() in {
                a.method();  
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Could not find data member field".to_string()]
    );
}

#[test]
fn declaration_of_type_with_self_inherited_access_wrong() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A { 
                field = 3; 
            } 
            type B inherits A  {
                method() => { self.field + 3; }; 
            } 
            
            let a = new B() in {
                a.method() && true;  
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Could not find data member field".to_string(),
            "Type mismatch: Cannot apply && to operands of type Number and Boolean".to_string()
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
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A(x: Boolean) { 
                field = x; 
            } 
            type B(x: Number) inherits A(2*x)  { 
            } 
            
            let a = new B(2) in {
                a;  
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Type A expects parameter 0 of type Boolean, but got Number".to_string()]
    );
}

#[test]
fn nested_super_constructor_call() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
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
            };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Could not find data member field".to_string(),
            "Could not find data member field2".to_string(),
            "Cannot access member field3 of type C. Properties are private, even to inherited types.".to_string()
        ]
    );
}

#[test]
fn dassigning_to_self() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A { 
                method() => { self := 5; };
            }
            ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Semantic Error: `self` is not a valid assignment target".to_string()]
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
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A {
                array: Number  = [1];
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

    assert_eq!(
        result.err().unwrap(),
        vec!["Type mismatch: Cannot assign Number* to Number".to_string()]
    );
}

#[test]
fn operation_on_element_without_indexing() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A {
                array: Number*  = [1];
                getArray() => { self.array; };
            }
            let a: A* = [new A(), new A()] in {
                a[0].getArray() + 1;
            };
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Type mismatch: Cannot apply + to operands of type Number* and Number".to_string()]
    );
}

#[test]
fn unknown_annotation() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            let a: Boniato = 1 in {a;};
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Semantic Error: Type or protocol Boniato is not defined.".to_string()]
    );
}

#[test]
fn unknown_annotation_in_method_param() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A {
                method(x: Number, y: Boniato, z: Malanga): Number {
                    x;
                }
            }
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Semantic Error: Type or protocol Boniato is not defined.".to_string(),
            "Semantic Error: Type or protocol Malanga is not defined.".to_string()
        ]
    );
}

#[test]
fn unknown_annotation_in_method_return() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A {
                method(x: Number, y: Number, z: Number): Boniato {
                    x;
                }
            }
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Semantic Error: Type or protocol Boniato is not defined.".to_string(),]
    );
}

#[test]
fn unknown_annotation_in_type_arg() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A(x: Boniato, y: Number){}
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Semantic Error: Type or protocol Boniato is not defined.".to_string(),]
    );
}

#[test]
fn reassign_nonexisting_property() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
            type A {
                x=3;
            }
            let a = new A() in {
                a.y := 1;
            };
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Could not find data member y".to_string()]
    );
}
