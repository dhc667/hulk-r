use ast::{
    DefinitionVisitor, TypeName, VisitableDefinition,
    typing::{BuiltInType, Type},
};
use generator::context::Context;

use crate::{DefinedTypeInfo, DefinitionInfo, FuncInfo, TypeInfo};
use std::collections::HashMap;

pub struct TypeDefinerVisitor<'a> {
    /// # Description
    ///
    /// This is a visitor that defines types in the global context. It only looks at the names of the types. Setting the inheritance relationships
    /// between types and vitisiting fields and functions of the types is left for another visitor. This aims to solve the problem of recursive types,
    /// allowing the use of the type before it is defined, types that reference each other in a recursive manner, etc.
    pub type_definitions: &'a mut Context<TypeInfo>,
    pub var_definitions: &'a mut Context<DefinitionInfo>,
    pub func_defintions: &'a mut Context<FuncInfo>,
    pub errors: &'a mut Vec<String>,
}

impl<'a> TypeDefinerVisitor<'a> {
    pub fn new(
        type_definitions: &'a mut Context<TypeInfo>,
        var_definitions: &'a mut Context<DefinitionInfo>,
        func_defintions: &'a mut Context<FuncInfo>,
        errors: &'a mut Vec<String>,
    ) -> Self {
        let instance = TypeDefinerVisitor {
            type_definitions,
            var_definitions,
            func_defintions,
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
                .type_definitions
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
        if self.type_definitions.is_defined(&type_name) {
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

        let methods_info: HashMap<_, _> = node
            .function_member_defs
            .iter()
            .map(|member| {
                (
                    member.identifier.id.clone(),
                    FuncInfo::from_func_def(member),
                )
            })
            .collect();
        let type_def = DefinedTypeInfo::new(node.name.clone(), members_info, methods_info);
        self.type_definitions
            .define(type_name, TypeInfo::Defined(type_def));
    }

    fn visit_function_def(&mut self, node: &mut ast::GlobalFunctionDef) -> () {
        // Define function information in global context
        let func_info = FuncInfo::from_func_def(&node.function_def);
        self.func_defintions
            .define(node.function_def.identifier.id.clone(), func_info.clone());

        // Define implicit type Wrapper in global context
        let type_wrapper_name = TypeName {
            id: FuncInfo::get_type_wrapper_name(&func_info),
            position: node.function_def.identifier.position.clone(),
        };
        let mut methods_info: HashMap<String, FuncInfo> = HashMap::new();
        methods_info.insert("invoke".to_string(), func_info.clone());
        let type_wrapper_def =
            DefinedTypeInfo::new(type_wrapper_name.clone(), HashMap::new(), methods_info);
        self.type_definitions.define(
            FuncInfo::get_type_wrapper_name(&func_info),
            TypeInfo::Defined(type_wrapper_def),
        );

        // Define implicit instance of the type
        self.var_definitions.define(
            FuncInfo::get_var_instance_name(&func_info),
            DefinitionInfo::new_from_identifier(
                &node.function_def.identifier,
                false,
                Some(Type::Defined(type_wrapper_name)),
            ),
        );
    }

    fn visit_constant_def(&mut self, node: &mut ast::ConstantDef) -> () {
        self.var_definitions.define(
            node.identifier.id.clone(),
            DefinitionInfo::new_from_identifier(&node.identifier, true, None),
        );
    }

    fn visit_protocol_def(&mut self, _node: &mut ast::ProtocolDef) -> () {
        todo!()
    }
}
