use ast::{DefinitionVisitor, VisitableDefinition, typing::BuiltInType};
use generator::context::Context;

use crate::{DefinedTypeInfo, DefinitionInfo, TypeInfo};
use std::collections::HashMap;

pub struct TypeDefinerVisitor<'a> {
    /// # Description
    ///
    /// This is a visitor that defines types in the global context. It only looks at the names of the types. Setting the inheritance relationships
    /// between types and vitisiting fields and functions of the types is left for another visitor. This aims to solve the problem of recursive types,
    /// allowing the use of the type before it is defined, types that reference each other in a recursive manner, etc.
    pub definitions: &'a mut Context<TypeInfo>,
    pub errors: &'a mut Vec<String>,
}

impl<'a> TypeDefinerVisitor<'a> {
    pub fn new(definitions: &'a mut Context<TypeInfo>, errors: &'a mut Vec<String>) -> Self {
        let instance = TypeDefinerVisitor {
            definitions,
            errors,
        };
        let built_ins = vec![
            BuiltInType::String,
            BuiltInType::Bool,
            BuiltInType::Number,
            BuiltInType::Object,
        ];
        for ty in built_ins {
            instance
                .definitions
                .define(ty.to_string(), TypeInfo::BuiltIn(ty));
        }
        return instance;
    }
}

impl<'a> DefinitionVisitor<()> for TypeDefinerVisitor<'a> {
    fn visit_definition(&mut self, node: &mut ast::Definition) -> () {
        node.accept(self);
    }

    fn visit_type_def(&mut self, node: &mut ast::TypeDef) -> () {
        let type_name = node.name.id.clone();
        if self.definitions.is_defined(&type_name) {
            self.errors
                .push(format!("Type {} is already defined", type_name));
            return;
        }
        let members_info: HashMap<_, _> = node
            .data_member_defs
            .iter()
            .map(|member| {
                (
                    member.identifier.id.clone(),
                    DefinitionInfo::new_from_identifier(&member.identifier, false, None),
                )
            })
            .collect();
        let type_def = DefinedTypeInfo::new(node.name.clone(), members_info);
        self.definitions
            .define(type_name, TypeInfo::Defined(type_def));
    }

    fn visit_function_def(&mut self, _node: &mut ast::GlobalFunctionDef) -> () {}

    fn visit_constant_def(&mut self, _node: &mut ast::ConstantDef) -> () {}

    fn visit_protocol_def(&mut self, _node: &mut ast::ProtocolDef) -> () {
        todo!()
    }
}
