use ast::{
    DefinitionVisitor, TypeName, VisitableDefinition,
    typing::{BuiltInType, Type, TypeAnnotation},
};
use error_handler::error::{
    error::HulkError,
    semantic::{
        function::FuncAlreadyDefined,
        type_definition::{TypeMemberAlreadyDefined, TypeOrProtocolAlreadyDefined},
    },
};
use generator::context::Context;

use std::collections::HashMap;

use crate::def_info::{DefinedTypeInfo, DefinitionInfo, FuncInfo, TypeInfo, VarInfo};

/// # Description
/// Defines types, functions and protocols in the global context. It only looks at the names of the types.
/// Setting the inheritance relationship between types and vitisiting fields and functions of the types is left for another visitor.
/// This aims to solve the problem of recursive types, allowing the use of the type before it is defined, types that reference each
/// other in a recursive manner, etc.
pub struct GlobalDefinerVisitor<'a> {
    pub type_definitions: &'a mut Context<TypeInfo>,
    pub var_definitions: &'a mut Context<VarInfo>,
    pub func_defintions: &'a mut Context<FuncInfo>,
    pub errors: &'a mut Vec<HulkError>,
}

impl<'a> GlobalDefinerVisitor<'a> {
    pub fn new(
        type_definitions: &'a mut Context<TypeInfo>,
        var_definitions: &'a mut Context<VarInfo>,
        func_defintions: &'a mut Context<FuncInfo>,
        errors: &'a mut Vec<HulkError>,
    ) -> Self {
        let instance = GlobalDefinerVisitor {
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

impl<'a> DefinitionVisitor<()> for GlobalDefinerVisitor<'a> {
    fn visit_definition(&mut self, node: &mut ast::Definition) -> () {
        node.accept(self);
    }

    fn visit_type_def(&mut self, node: &mut ast::TypeDef) -> () {
        let type_name = node.name.id.clone();
        if self.type_definitions.is_defined(&type_name) {
            self.errors.push(
                TypeOrProtocolAlreadyDefined::new(
                    type_name.clone(),
                    node.name.position.start.clone(),
                )
                .into(),
            );
            return;
        }

        let mut members_info: HashMap<String, DefinitionInfo> = HashMap::new();

        for member in &node.data_member_defs {
            let member_name = member.identifier.id.clone();
            if members_info.contains_key(&member_name) {
                self.errors.push(
                    TypeMemberAlreadyDefined::new(
                        member_name.clone(),
                        type_name.clone(),
                        member.identifier.position.start.clone(),
                    )
                    .into(),
                );
                continue;
            }
            members_info.insert(
                member_name,
                DefinitionInfo::Var(VarInfo::new_from_identifier(
                    &member.identifier,
                    false,
                    None,
                )),
            );
        }

        for member in &node.function_member_defs {
            let member_name = member.identifier.id.clone();
            if members_info.contains_key(&member_name) {
                self.errors.push(
                    TypeMemberAlreadyDefined::new(
                        member_name.clone(),
                        type_name.clone(),
                        member.identifier.position.start.clone(),
                    )
                    .into(),
                );
                continue;
            }
            members_info.insert(member_name, DefinitionInfo::Func(FuncInfo::from(member)));
        }

        let arguments_types: Vec<TypeAnnotation> = node
            .parameter_list
            .iter()
            .map(|id| id.info.ty.clone())
            .collect();
        let type_def = DefinedTypeInfo::new(node.name.clone(), members_info, arguments_types);
        self.type_definitions
            .define(type_name, TypeInfo::Defined(type_def));
    }

    fn visit_function_def(&mut self, node: &mut ast::GlobalFunctionDef) -> () {
        // Check if the function is already defined
        if self
            .func_defintions
            .is_defined(&node.function_def.identifier.id)
        {
            self.errors.push(
                FuncAlreadyDefined::new(
                    node.function_def.identifier.id.clone(),
                    node.function_def.identifier.position.start.clone(),
                )
                .into(),
            );
            return;
        }

        // Define function information in global context
        let func_info = FuncInfo::from(&node.function_def);
        self.func_defintions
            .define(node.function_def.identifier.id.clone(), func_info.clone());

        // Define implicit type Wrapper in global context
        let type_wrapper_name = TypeName {
            id: FuncInfo::get_type_wrapper_name(&func_info),
            position: node.function_def.identifier.position.clone(),
        };
        let methods_info: HashMap<String, DefinitionInfo> = ["invoke"]
            .iter()
            .map(|&name| (name.to_string(), DefinitionInfo::Func(func_info.clone())))
            .collect();
        let type_wrapper_def =
            DefinedTypeInfo::new(type_wrapper_name.clone(), methods_info, vec![]);
        self.type_definitions.define(
            FuncInfo::get_type_wrapper_name(&func_info),
            TypeInfo::Defined(type_wrapper_def),
        );

        // Define implicit instance of the type
        self.var_definitions.define(
            FuncInfo::get_var_instance_name(&func_info),
            VarInfo::new(
                node.function_def.identifier.id.clone(),
                true,
                node.function_def.identifier.position.clone(),
                Some(Type::Defined(type_wrapper_name)),
            ),
        );
    }

    fn visit_constant_def(&mut self, _node: &mut ast::ConstantDef) -> () {}

    fn visit_protocol_def(&mut self, _node: &mut ast::ProtocolDef) -> () {
        todo!()
    }
}
