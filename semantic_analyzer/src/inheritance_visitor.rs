use std::collections::HashMap;

use ast::{
    ConstantDef, DefinitionVisitor, GlobalFunctionDef, ProtocolDef, TypeDef, VisitableDefinition,
    typing::{BuiltInType, Type, TypeAnnotation},
};
use generator::context::Context;

use crate::def_info::TypeInfo;

pub struct InheritanceVisitor<'a> {
    pub type_hierarchy: &'a mut HashMap<String, TypeAnnotation>,
    pub type_definitions: &'a mut Context<TypeInfo>,
    pub errors: &'a mut Vec<String>,
}

impl<'a> InheritanceVisitor<'a> {
    pub fn new(
        type_hierarchy: &'a mut HashMap<String, TypeAnnotation>,
        type_definitions: &'a mut Context<TypeInfo>,
        errors: &'a mut Vec<String>,
    ) -> Self {
        let instance = InheritanceVisitor {
            type_hierarchy,
            type_definitions,
            errors,
        };
        // We asume that the built-in types are already defined in the context
        let built_ins = vec![
            Type::BuiltIn(BuiltInType::String),
            Type::BuiltIn(BuiltInType::Bool),
            Type::BuiltIn(BuiltInType::Number),
            Type::BuiltIn(BuiltInType::Object),
        ];
        for ty in built_ins {
            match ty {
                Type::BuiltIn(BuiltInType::Object) => {
                    instance.type_hierarchy.insert(ty.to_string(), None);
                }
                _ => {
                    instance
                        .type_hierarchy
                        .insert(ty.to_string(), Some(Type::BuiltIn(BuiltInType::Object)));
                }
            }
        }
        return instance;
    }

    /// # Description
    /// Checks if the type hierarchy has cycles. if it does, it returns the class involved in the cycle.
    pub fn has_cycles(&self) -> Option<Vec<String>> {
        let mut visiting: HashMap<String, bool> = HashMap::new();
        let mut tree_path: Vec<String> = Vec::new();
        for (node, _) in self.type_hierarchy.iter() {
            let node_result = self.has_cycles_helper(node, &mut visiting, &mut tree_path);
            match node_result {
                Some(path) => {
                    return Some(path);
                }
                _ => {}
            }
        }
        None
    }

    fn has_cycles_helper(
        &self,
        node: &str,
        visiting: &mut HashMap<String, bool>,
        tree_path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        if let Some(&is_visiting) = visiting.get(node) {
            if !is_visiting {
                return None;
            }

            for i in 0..tree_path.len() {
                if tree_path[i] == node {
                    let mut cycle = tree_path[i..].to_vec();
                    cycle.push(node.to_string());
                    return Some(cycle);
                }
            }
        }
        tree_path.push(node.to_string());
        visiting.insert(node.to_string(), true);
        if let Some(parent) = self.type_hierarchy.get(node) {
            if let Some(parent_type) = parent {
                let node_result =
                    self.has_cycles_helper(&parent_type.to_string(), visiting, tree_path);
                match node_result {
                    Some(path) => {
                        return Some(path);
                    }
                    _ => {}
                }
            }
        }
        tree_path.pop();
        visiting.insert(node.to_string(), false);
        None
    }
}

impl<'a> DefinitionVisitor<()> for InheritanceVisitor<'a> {
    fn visit_definition(&mut self, node: &mut ast::Definition) -> () {
        node.accept(self);
    }

    fn visit_type_def(&mut self, node: &mut TypeDef) -> () {
        match &node.inheritance_indicator {
            Some(inheritance) => {
                let parent_name = inheritance.parent_name.id.clone();
                let class_name = node.name.id.clone();

                match self.type_definitions.get_value(&parent_name) {
                    Some(parent_type) => match parent_type {
                        TypeInfo::BuiltIn(_) => {
                            self.errors.push(format!(
                                "Type {} is a built-in type and cannot be inherited from",
                                parent_name
                            ));
                        }
                        TypeInfo::Defined(parent_def) => {
                            self.type_hierarchy
                                .insert(class_name, Some(Type::Defined(parent_def.name.clone())));
                        }
                    },
                    None => {
                        self.errors
                            .push(format!("Type {} is not defined", parent_name));
                    }
                }
            }
            None => {
                self.type_hierarchy.insert(
                    node.name.id.clone(),
                    Some(Type::BuiltIn(BuiltInType::Object)),
                );
            }
        }
    }

    fn visit_function_def(&mut self, _node: &mut GlobalFunctionDef) -> () {}

    fn visit_constant_def(&mut self, _node: &mut ConstantDef) -> () {}

    fn visit_protocol_def(&mut self, _node: &mut ProtocolDef) -> () {
        todo!()
    }
}
