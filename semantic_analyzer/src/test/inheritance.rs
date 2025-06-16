use std::collections::HashMap;

use ast::{
    Definition,
    typing::{BuiltInType, Type},
};
use error_handler::error_handler::ErrorHandler;
use generated_parser::ProgramParser;

use crate::semantic_analyzer::SemanticAnalyzer;

#[test]
fn built_ins_inheritance() {
    let p = ProgramParser::new();

    let mut answ = p.parse("3;").unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("String"),
        true
    );
    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("Boolean"),
        true
    );

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("Number"),
        true
    );

    assert_eq!(
        semantic_analyzer.type_definitions.is_defined("Object"),
        true
    );

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("String")
            .unwrap()
            .clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("Boolean")
            .unwrap()
            .clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("Number")
            .unwrap()
            .clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("Object")
            .unwrap()
            .clone(),
        None,
    );

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn basic_inheritance() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "type A {} 
        type B inherits A {}",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("A"), true);

    assert_eq!(
        semantic_analyzer.type_hierarchy.get("A").unwrap().clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("B"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("B")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "A".to_string(),
    );

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn before_delcared() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
          type B inherits A {}
          type A {} 
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("A"), true);

    assert_eq!(
        semantic_analyzer.type_hierarchy.get("A").unwrap().clone(),
        Some(Type::BuiltIn(BuiltInType::Object)),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("B"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("B")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "A".to_string(),
    );

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn nested_inheritance() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
          type B inherits A {}
          type A inherits C {}
          type C {} 
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("A"), true);

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("A")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "C".to_string(),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("B"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("B")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "A".to_string(),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("C"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("C")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "Object".to_string(),
    );

    assert!(result.is_ok(), "Errors: {:?}", result.err());
}

#[test]
fn inheritance_cycle() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
          type B inherits A {}
          type A inherits B {} 
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    assert_eq!(semantic_analyzer.type_definitions.is_defined("A"), true);

    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("A")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "B".to_string(),
    );

    assert_eq!(semantic_analyzer.type_definitions.is_defined("B"), true);
    assert_eq!(
        semantic_analyzer
            .type_hierarchy
            .get("B")
            .unwrap()
            .clone()
            .unwrap()
            .to_string(),
        "A".to_string(),
    );

    println!("{:?}", semantic_analyzer.errors);

    assert!(result.is_err(), "Errors: {:?}", result.err());
}

#[test]
fn complicated_inheritance_cycle() {
    let p = ProgramParser::new();

    let mut answ = p
        .parse(
            "
          type A inherits B {}
          type B inherits C {}
          type C inherits D {}
          type D inherits E {}
          type E inherits F {}
          type F inherits G {}
          type G inherits H {}
          type H inherits I {}
          type I inherits J {}
          type J inherits K {}
          type K inherits L {}
          type L inherits M {}
          type M inherits N {}
          type N inherits A {}
          type P inherits Q {}
          type W inherits O {}
          type O inherits Y {}
          type Q inherits W {}
          type Z inherits W {}
          type Y { message = \"boniato\";}
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let result = semantic_analyzer.analyze_program_ast(&mut answ);

    println!("{:?}", semantic_analyzer.errors);

    assert!(result.is_err(), "Errors: {:?}", result.err());
}

#[test]
fn redeclare_object() {
    let program = "
        type Object {}";

    let mut error_handler = ErrorHandler::new(program);
    let p = ProgramParser::new();
    let mut answ = p.parse(program).unwrap();
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze_program_ast(&mut answ)
        .expect_err("Should return an error");
    error_handler.extend_errors(semantic_analyzer.errors);

    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Semantic Error: Already exists a type or protocol `Object`."]
    )
}

#[test]
fn sort_definitions() {
    let p = Parser::new();

    let mut answ = p
        .parse(
            "
          type B inherits A {}
          type A {}
          type C inherits B {}
          type D inherits C {}
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let definitions = answ.definitions;

    let type_indexes = definitions
        .iter()
        .enumerate()
        .filter_map(|(i, def)| match def {
            Definition::TypeDef(type_def) => Some((type_def.name.id.clone(), i)),
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    assert_eq!(type_indexes.len(), 4);
    assert!(type_indexes["A"] < type_indexes["B"]);
    assert!(type_indexes["A"] < type_indexes["C"]);
    assert!(type_indexes["A"] < type_indexes["D"]);
}

#[test]
fn sort_definitions2() {
    let p = Parser::new();

    let mut answ = p
        .parse(
            "
          type B inherits A {}
          type A {}
          type C inherits B {}
          type D inherits C {}
          type E inherits D {}
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let definitions = answ.definitions;

    let type_indexes = definitions
        .iter()
        .enumerate()
        .filter_map(|(i, def)| match def {
            Definition::TypeDef(type_def) => Some((type_def.name.id.clone(), i)),
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    assert_eq!(type_indexes.len(), 5);
    assert!(type_indexes["A"] < type_indexes["B"]);
    assert!(type_indexes["A"] < type_indexes["C"]);
    assert!(type_indexes["A"] < type_indexes["D"]);
    assert!(type_indexes["A"] < type_indexes["E"]);
}

#[test]
fn sort_definitions3() {
    let p = Parser::new();

    let mut answ = p
        .parse(
            "
          type Shirt inherits Hoodie {}
          type Underwear inherits Pants {}
          type Shoes inherits School {}
          type School {}
          type Socks inherits Shoes {}
          
          function foo(): Number {
            return 3;
          }
          
          type Hoodie inherits School {}
          type Pants inherits Belt {}
          type Belt {}
          ",
        )
        .unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze_program_ast(&mut answ).unwrap();

    let definitions = answ.definitions;

    let type_indexes = definitions
        .iter()
        .enumerate()
        .filter_map(|(i, def)| match def {
            Definition::TypeDef(type_def) => Some((type_def.name.id.clone(), i)),
            Definition::FunctionDef(func_def) => {
                Some((func_def.function_def.identifier.id.clone(), i))
            }
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    assert!(type_indexes["School"] < type_indexes["Hoodie"]);
    assert!(type_indexes["School"] < type_indexes["Shoes"]);
    assert!(type_indexes["School"] < type_indexes["Socks"]);
    assert!(type_indexes["School"] < type_indexes["Shirt"]);

    assert!(type_indexes["Hoodie"] < type_indexes["Shirt"]);
    assert!(type_indexes["Shoes"] < type_indexes["Socks"]);

    assert!(type_indexes["Belt"] < type_indexes["Pants"]);
    assert!(type_indexes["Pants"] < type_indexes["Underwear"]);

    assert!(
        type_indexes
            .iter()
            .all(|(_, &index)| { index <= type_indexes["foo"] }),
        "Function `foo` should be the last definition"
    );
}
