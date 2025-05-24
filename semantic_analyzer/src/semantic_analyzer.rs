use std::collections::HashMap;

use ast::{VisitableDefinition, VisitableExpression, typing::TypeAnnotation};
use generator::context::Context;

use crate::{
    InheritanceVisitor,
    def_info::{FuncInfo, TypeInfo, VarInfo},
    type_definer_visitor::TypeDefinerVisitor,
};

use super::SemanticVisitor;

pub struct SemanticAnalyzer {
    pub type_definitions: Context<TypeInfo>,
    pub type_hierarchy: HashMap<String, TypeAnnotation>,
    pub func_definitions: Context<FuncInfo>,
    pub var_definitions: Context<VarInfo>,
    pub errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            type_definitions: Context::new_one_frame(),
            type_hierarchy: HashMap::new(),
            var_definitions: Context::new_one_frame(),
            func_definitions: Context::new_one_frame(),
            errors: Vec::new(),
        }
    }

    pub fn analyze_program_ast(&mut self, program: &mut ast::Program) -> Result<(), Vec<String>> {
        // Define types in the global context
        let mut type_definer_visitor = TypeDefinerVisitor::new(
            &mut self.type_definitions,
            &mut self.var_definitions,
            &mut self.func_definitions,
            &mut self.errors,
        );

        for definition in &mut program.definitions {
            definition.accept(&mut type_definer_visitor);
        }

        // Define inheritance relationships
        let mut inheritance_visitor = InheritanceVisitor::new(
            &mut self.type_hierarchy,
            &mut self.type_definitions,
            &mut self.errors,
        );
        for definition in &mut program.definitions {
            definition.accept(&mut inheritance_visitor);
        }

        // Check for cycles in the inheritance graph
        if let Some(cycle) = inheritance_visitor.has_cycles() {
            self.errors.push(format!(
                "Inheritance cycle detected: {:?}",
                cycle.join(" -> ")
            ));
            return Err(self.errors.clone());
        }

        // We return here to avoid running semantic checks on undefined stuff
        if self.errors.len() > 0 {
            return Err(self.errors.clone());
        }

        let mut semantic_visitor = SemanticVisitor::new(
            &mut self.type_definitions,
            &mut self.type_hierarchy,
            &mut self.var_definitions,
            &mut self.func_definitions,
            &mut self.errors,
        );
        for definition in &mut program.definitions {
            definition.accept(&mut semantic_visitor);
        }

        for expression in &mut program.expressions {
            expression.accept(&mut semantic_visitor);
        }

        if self.errors.len() > 0 {
            return Err(self.errors.clone());
        }

        Ok(())
    }
}
