use ast::VisitableExpression;

use super::SemanticVisitor;

pub struct SemanticAnalyzer {}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn analyze_program_ast(&self, program: &mut ast::Program) -> Result<(), Vec<String>>{
        let mut visitor = SemanticVisitor::new();

        if program.definitions.len() > 0 {
            todo!();
        }

        program.main_expression.accept(&mut visitor);

        if visitor.errors.len() > 0 {
            return Err(visitor.errors);
        }

        Ok(())
    }
}

