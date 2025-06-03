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
        
        program += &definitions_code;
        program += "define i32 @main() {\nentry:\n";

        let mut expressions_code = String::new();
        if !node.expressions.is_empty() {
            let inner = node.expressions[0].accept(&mut generator);
            expressions_code += &inner.preamble;
        }
        
        program += &expressions_code;
        program += "\nret i32 0\n}\n";

        program
    }
}

