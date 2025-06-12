use ast::{VisitableExpression, VisitableDefinition};

use crate::visitor::GeneratorVisitor;


pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn generate_code_from_program_ast(self, node: &mut ast::Program) -> String {
        let mut generator = GeneratorVisitor::new();
        let mut program = generator.instantiate_global_print_helpers();
        
        
        let mut definitions_code = String::new();
        for definition in &mut node.definitions {
            let definition_result = definition.accept(&mut generator);
            definitions_code += &definition_result.preamble;
        }
        
        

        let mut expressions_code = String::new();
        if !node.expressions.is_empty() {
            let inner = node.expressions[0].accept(&mut generator);
            expressions_code += &inner.preamble;
        }

        let mut global_String = String::new();
        for string_global in generator.string_constants.iter() {
            global_String.push_str(&format!("{}\n", string_global.clone()));
        }
        let global_str: &str = global_String.as_str();
        println!("global_str: {global_str}");
        program += global_str;
        program += &definitions_code;
        program += "define i32 @main() {\nentry:\n";
        program += &expressions_code;
        program += "\nret i32 0\n}\n";

        program
    }
}

