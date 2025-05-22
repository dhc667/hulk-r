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

    assert!(result.is_ok(), "Errors: {:?}", result.err());
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

    assert!(result.is_ok(), "Errors: {:?}", result.err());
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

    assert!(result.is_ok(), "Errors: {:?}", result.err());
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

    assert!(result.is_ok(), "Errors: {:?}", result.err());
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
