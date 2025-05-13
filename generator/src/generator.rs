use ast::VisitableExpression;

use crate::visitor::GeneratorVisitor;


pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn generate_code_from_program_ast(self, node: &mut ast::Program) -> String {
        if node.definitions.len() > 0 {
            todo!();
        }

        let mut expression_generator = GeneratorVisitor::new();

        let mut program =
            expression_generator.instantiate_global_print_helpers() + "define i32 @main() {\nentry:\n";

        if node.expressions.len() != 1 {
            todo!()
        }

        let inner = node.expressions[0].accept(&mut expression_generator);

        program = program + &inner.preamble;

        program = program + "\nret i32 0\n}\n";

        program
    }
}

