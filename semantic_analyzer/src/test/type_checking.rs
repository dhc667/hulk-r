use ast::typing::{BuiltInType, Type, to_string};
use parser::parser::Parser;

use crate::semantic_analyzer::SemanticAnalyzer;

#[test]
pub fn simple_typing() {
    let p = Parser::new();
    let mut answ = p.parse("let x = 1 in { x + 1 ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    let expression = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .body
        .as_block()
        .unwrap()
        .body_items[0]
        .as_expression()
        .unwrap();

    let var = expression.as_bin_op().unwrap().lhs.as_variable().unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::Number)));
    assert_eq!(var.info.ty, Some(Type::BuiltIn(BuiltInType::Number)))
}

#[test]
pub fn binary_op_error() {
    let p = Parser::new();
    let mut answ = p.parse("let x = 1 in { x + true ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot apply + to operands of type Number and Boolean".to_string()]
    );
}

#[test]
pub fn unary_op_error() {
    let p = Parser::new();
    let mut answ = p.parse("let x = true in { -x ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot apply - to operand of type Boolean".to_string()]
    );
}

#[test]
pub fn dassing_error() {
    let p = Parser::new();
    let mut answ = p.parse("let x = true in { x:=3 ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: x is Boolean but is being reassigned with Number".to_string()]
    );
}

#[test]
pub fn simple_inference_test() {
    let p = Parser::new();
    let mut answ = p
        .parse("let x = if (true) true else 3 in { x + 1 ;};")
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot apply + to operands of type Object and Number".to_string()]
    );
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::Object)));
}

#[test]
pub fn nested_inference() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
        let result = 
            let x = 1 in { 
                let y = 1 > 0 in { 
                    if (y == true) x else 0; 
                } ;
            } 
            in {result;};",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);

    let outer_let_in = answ.expressions[0].as_let_in().unwrap();

    let expr_type = outer_let_in.assignment.identifier.info.ty.clone();

    let let_in = outer_let_in.assignment.rhs.as_let_in().unwrap();

    let dec_id = &let_in.assignment.identifier;

    let inner_let_in = let_in.body.as_block().unwrap().body_items[0]
        .as_expression()
        .unwrap()
        .as_let_in()
        .unwrap();

    let inner_dec_id = &inner_let_in.assignment.identifier;

    assert_eq!(dec_id.info.ty, Some(Type::BuiltIn(BuiltInType::Number)));
    assert_eq!(inner_dec_id.info.ty, Some(Type::BuiltIn(BuiltInType::Bool)));
    assert_eq!(expr_type, Some(Type::BuiltIn(BuiltInType::Number)));
}

#[test]
pub fn string_typing() {
    let p = Parser::new();
    let mut answ = p.parse("let x = \"boniato\" in { x ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::String)));
}

#[test]
pub fn list_typing() {
    let p = Parser::new();
    let mut answ = p.parse("let x = [1, 2, 3] in { x ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(
        dec.info.ty,
        Some(Type::Iterable(Box::new(Type::BuiltIn(BuiltInType::Number))))
    );
}

#[test]
pub fn list_typing_2() {
    let p = Parser::new();
    let mut answ = p.parse("let x = [1, true, \"hola\"] in x;").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let assignment = &answ.expressions[0].as_let_in().unwrap().assignment;

    let dec = &assignment.identifier;
    let list_literal = assignment.rhs.as_list_literal().unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(
        dec.info.ty,
        Some(Type::Iterable(Box::new(Type::BuiltIn(BuiltInType::Object))))
    );
    assert_eq!(
        list_literal.list_type,
        Some(Type::Iterable(Box::new(Type::BuiltIn(BuiltInType::Object))))
    )
}

#[test]
pub fn list_indexing() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
        let x = [1, 2, 3] in x[0];",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let indexing = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .body
        .as_list_indexing()
        .unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(
        indexing.list_type,
        Some(Type::Iterable(Box::new(Type::BuiltIn(BuiltInType::Number))))
    );
}

#[test]
pub fn list_indexing_2() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
            type A {
                x = [1, 2, 3];
                method(): Number* => self.x;
            }
            let a = new A() in a.method()[0];",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let indexing = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .body
        .as_list_indexing()
        .unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(
        indexing.list_type,
        Some(Type::Iterable(Box::new(Type::BuiltIn(BuiltInType::Number))))
    );
}

#[test]
pub fn list_typing_with_different_types() {
    let p = Parser::new();
    let mut answ = p.parse("let x = [1, 2, \"3\"] in { x ;};").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(
        dec.info.ty,
        Some(Type::Iterable(Box::new(Type::BuiltIn(BuiltInType::Object))))
    );
}

#[test]
pub fn list_indexing_typing() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
        let result = let x = [1, 2, 3] in x[0] in {
            result;
        };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let dec = &answ.expressions[0]
        .as_let_in()
        .unwrap()
        .assignment
        .identifier;

    assert_eq!(semantic_analyzer.errors.len(), 0);
    assert_eq!(dec.info.ty, Some(Type::BuiltIn(BuiltInType::Number)));
}

#[test]
pub fn list_indexing_typing_error() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
        let result = let x = [1, 2, 3] in x[true] in {
            result;
        };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot use index of type Boolean to access iterable".to_string()]
    );
}

#[test]
pub fn list_indexing_typing_error_2() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
        let result = let x = [1, 2, 3] in let y = true in x[true] in {
            result;
        };",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");

    assert_eq!(
        semantic_analyzer.errors,
        vec!["Type mismatch: Cannot use index of type Boolean to access iterable".to_string()]
    );
}

#[test]
pub fn call_var_with_method_name() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
            type A {
                method(): Number {
                    return method + 1;
                }
            }
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert!(result.is_err(), "Errors {:?}", result.err())
}

#[test]
pub fn iterate_non_iterable() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
            for(a in 3){
                a;
            };
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec!["Semantic Error: Cannot iterate over type Number".to_string()]
    )
}

#[test]
pub fn annotate_field_accessed_object() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
            type A{
                x = 3;
                method(): Number => self.x;
            }
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let self_type = answ.definitions[0]
        .as_type_def()
        .unwrap()
        .function_member_defs[0]
        .body
        .as_arrow_expression()
        .unwrap()
        .expression
        .as_data_member_access()
        .unwrap()
        .obj_type
        .clone();

    assert_eq!(to_string(&self_type), "A".to_string());
}

#[test]
pub fn annotate_function_accessed_object() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
            type A{
                method(): Number => 3;
            }
            let a = new A() in a.method();
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let a_type = answ.expressions[0]
        .as_let_in()
        .unwrap()
        .body
        .as_function_member_access()
        .unwrap()
        .obj_type
        .clone();

    assert_eq!(to_string(&a_type), "A".to_string());
}

#[test]
pub fn unknown_annotation_in_constructor_called() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
            type Point(a: Number, b: Bool){x=a;y=b;}
            let a = new Point(4, false) in 
                let x = if(a.y) 1 else 2 in 
                    x;
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Semantic Error: Type or protocol Bool is not defined.",
            "Cannot access member y of type Point. Properties are private, even to inherited types."
        ]
    )
}

#[test]
pub fn unknown_annotation_in_func_called() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
            function foo(a: Number, b: Bool): Number => a;
            foo(1, false) + true;
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Semantic Error: Type or protocol Bool is not defined.",
            "Type mismatch: Cannot apply + to operands of type Number and Boolean"
        ]
    )
}

#[test]
pub fn unknown_annotation_in_method_called() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            "
            type A{
                method(x: Bool): Number => 3;
            }
            let a = new A() in 
                a.method(true) + false;
        ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Semantic Error: Type or protocol Bool is not defined.",
            "Type mismatch: Cannot apply + to operands of type Number and Boolean"
        ]
    )
}

#[test]
fn concat_checks1() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            r#"
            let x = "hello", y = " world" in {
                x @ y;
                x @@ y;
            };
        "#,
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);
}

#[test]
fn concat_checks2() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            r#"
            let x = "hello", y = 3 in {
                x @ y;
                x @@ y;
            };
        "#,
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);
}

#[test]
fn concat_checks3() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            r#"
            let x = 3, y = 4 in {
                x @ y;
                x @@ y;
            };
        "#,
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);
}

#[test]
fn concat_checks4() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            r#"
            let x = 3, y = true in {
                x @ y;
                x @@ y;
            };
        "#,
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);
}

#[test]
fn concat_checks5() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            r#"
            let x = true, y = false in {
                x @ y;
                x @@ y;
            };
        "#,
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    assert_eq!(semantic_analyzer.errors.len(), 0);
}

#[test]
fn concat_checks6() {
    let p = Parser::new();
    let mut answ = p
        .parse(
            r#"
            let x = [1, 2, 3], y = [4, 5] in {
                x @ y;
                x @@ y;
            };
        "#,
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        result.err().unwrap(),
        vec![
            "Type mismatch: Cannot apply @ to operands of type Number* and Number*",
            "Type mismatch: Cannot apply @@ to operands of type Number* and Number*"
        ]
    );
}
