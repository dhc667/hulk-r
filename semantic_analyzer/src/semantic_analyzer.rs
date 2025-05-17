use ast::{VisitableDefinition, VisitableExpression, typing::Type};
use generator::context::Context;

use crate::{DefinitionInfo, type_definer_visitor::TypeDefinerVisitor};

use super::SemanticVisitor;

pub struct SemanticAnalyzer {
    pub type_definitions: Context<Type>,
    pub var_definitions: Context<DefinitionInfo>,
    pub errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            type_definitions: Context::new_one_frame(),
            var_definitions: Context::new_one_frame(),
            errors: Vec::new(),
        }
    }

    pub fn analyze_program_ast(&mut self, program: &mut ast::Program) -> Result<(), Vec<String>> {
        let mut type_definer_visitor =
            TypeDefinerVisitor::new(&mut self.type_definitions, &mut self.errors);

        for definition in &mut program.definitions {
            definition.accept(&mut type_definer_visitor);
        }

        let mut semantic_visitor =
            SemanticVisitor::new(&mut self.var_definitions, &mut self.errors);
        for expression in &mut program.expressions {
            expression.accept(&mut semantic_visitor);
        }
        if self.errors.len() > 0 {
            return Err(self.errors.clone());
        }

        Ok(())
    }
}
