use ast::{VisitableDefinition, VisitableExpression};

use crate::type_definer_visitor::{self, TypeDefinerVisitor};

use super::SemanticVisitor;

pub struct SemanticAnalyzer {
    // Note: this members are public for testing purposes
    pub type_definer_visitor: TypeDefinerVisitor,
    pub semantic_visitor: SemanticVisitor,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            type_definer_visitor: TypeDefinerVisitor::new(),
            semantic_visitor: SemanticVisitor::new(),
        }
    }

    pub fn analyze_program_ast(&mut self, program: &mut ast::Program) -> Result<(), Vec<String>> {
        // TODO: maybe avoid cloning the errors
        let mut errors = Vec::new();
        for definition in &mut program.definitions {
            definition.accept(&mut self.type_definer_visitor);
        }
        if self.type_definer_visitor.errors.len() > 0 {
            errors.extend(self.type_definer_visitor.errors.clone());
        }

        for expression in &mut program.expressions {
            expression.accept(&mut self.semantic_visitor);
        }
        if self.semantic_visitor.errors.len() > 0 {
            errors.extend(self.semantic_visitor.errors.clone());
        }
        if errors.len() > 0 {
            return Err(errors.clone());
        }

        Ok(())
    }
}
