use std::collections::HashMap;

use ast::{
    ConstantDef, DefinitionVisitor, GlobalFunctionDef, ProtocolDef, TypeDef, VisitableDefinition,
    typing::{BuiltInType, Type, TypeAnnotation},
};
use generator::context::Context;

use crate::def_info::TypeInfo;

/// # Description
/// Visitor that sets the inheritance relationship between types.
/// It assumes that the built-in types are already defined in the context.
/// It also assumes that the type definitions are already defined in the context.
/// It does not visit fields or functions of the types, that is left for another visitor.
/// # Arguments
/// * `type_hierarchy` - A mutable reference to a HashMap that holds the inheritance relationship between types.
/// * `type_definitions` - A mutable reference to a context that holds the type definitions.
/// * `errors` - A mutable reference to a vector that holds the errors encountered during the visit.
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
