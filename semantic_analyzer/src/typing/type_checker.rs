use std::collections::HashMap;

use ast::{
    BinaryOperator, UnaryOperator,
    typing::{Type, TypeAnnotation, to_string},
};

use super::{get_binary_op_functor_type, get_unary_op_functor_type};
use crate::{
    def_info::{DefinedTypeInfo, FuncInfo},
    graph_utils::{lca::LCA, parent_map_to_adj},
};

pub struct TypeChecker {
    type_ids: HashMap<String, usize>,
    type_names: Vec<String>,
    type_definitions: HashMap<String, TypeAnnotation>,
    lca: LCA,
}

impl TypeChecker {
    pub fn new(
        type_hierarchy: &HashMap<String, TypeAnnotation>,
        type_definitions: HashMap<String, TypeAnnotation>,
    ) -> Self {
        let mut type_ids = HashMap::new();
        let mut type_names = Vec::new();
        for (i, type_name) in type_hierarchy.keys().enumerate() {
            type_ids.insert(type_name.clone(), i);
            type_names.push(type_name.clone());
        }
        let adj = parent_map_to_adj(&type_hierarchy, &type_ids);
        let object_name = Type::BuiltIn(ast::typing::BuiltInType::Object).to_string();
        let root = type_ids[&object_name];
        let lca = LCA::new(&adj, root);
        TypeChecker {
            type_ids,
            type_names,
            type_definitions,
            lca,
        }
    }

    /// # Description
    /// Converts a type to its id in the type tree graph
    /// Note: it asumes that ty is defined, will panic if it is not
    fn type_to_id(&self, ty: &Type) -> usize {
        let type_name = ty.to_string();
        let id = self.type_ids.get(&type_name);
        *id.unwrap()
    }

    pub fn conforms(&self, a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
        match (a, b) {
            (None, _) => return true,
            (_, None) => return true,
            (Some(a), Some(b)) => {
                let a_id = self.type_to_id(a);
                let b_id = self.type_to_id(b);
                if a_id == b_id {
                    return true;
                }
                let common = self.lca.get_lca(a_id, b_id);
                common == b_id
            }
        }
    }

    pub fn get_common_supertype(&self, a: &TypeAnnotation, b: &TypeAnnotation) -> TypeAnnotation {
        match (a, b) {
            (None, _) => return b.clone(),
            (_, None) => return a.clone(),
            (Some(a), Some(b)) => {
                let a_id = self.type_to_id(a);
                let b_id = self.type_to_id(b);
                let common = self.lca.get_lca(a_id, b_id);
                let common_name = self.type_names.get(common);
                if let Some(common_name) = common_name {
                    if let Some(common_type) = self.type_definitions.get(common_name) {
                        return common_type.clone();
                    }
                }
                None
            }
        }
    }

    pub fn check_bin_op(
        &self,
        op: &BinaryOperator,
        left: &TypeAnnotation,
        right: &TypeAnnotation,
        errors: &mut Vec<String>,
    ) -> TypeAnnotation {
        let functor = get_binary_op_functor_type(&op);

        if !self.conforms(&left, &functor.parameter_types[0])
            || !self.conforms(&right, &functor.parameter_types[1])
        {
            errors.push(format!(
                "Type mismatch: Cannot apply {} to operands of type {} and {}",
                op,
                to_string(&left),
                to_string(&right)
            ));
        }
        *functor.return_type.clone()
    }

    pub fn check_up_op(
        &self,
        op: &UnaryOperator,
        operand: &TypeAnnotation,
        errors: &mut Vec<String>,
    ) -> TypeAnnotation {
        let functor = get_unary_op_functor_type(&op);

        if !self.conforms(&operand, &functor.parameter_types[0]) {
            errors.push(format!(
                "Type mismatch: Cannot apply {} to operand of type {}",
                op,
                to_string(&operand)
            ));
        }
        *functor.return_type.clone()
    }

    pub fn check_functor_call(
        &self,
        fn_info: &FuncInfo,
        parameters: &Vec<TypeAnnotation>,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let functor = &fn_info.functor_type;
        if functor.parameter_types.len() != parameters.len() {
            errors.push(format!(
                "Function {} expects {} parameters, but {} were provided",
                fn_info.name,
                functor.parameter_types.len(),
                parameters.len()
            ));
            return Err(errors);
        }
        for (i, (expected, provided)) in functor
            .parameter_types
            .iter()
            .zip(parameters.iter())
            .enumerate()
        {
            if !self.conforms(expected, provided) {
                errors.push(format!(
                    "Function {} expects parameter {} of type {}, but got {}",
                    fn_info.name,
                    i,
                    to_string(expected),
                    to_string(provided)
                ));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn check_type_constructor(
        &self,
        type_definition: &DefinedTypeInfo,
        parameters: &Vec<TypeAnnotation>,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if type_definition.arguments_types.len() != parameters.len() {
            errors.push(format!(
                "Type {} has {} parameters, but {} were provided",
                type_definition.name.id,
                type_definition.arguments_types.len(),
                parameters.len()
            ));
            return Err(errors);
        }
        for (i, (expected, provided)) in type_definition
            .arguments_types
            .iter()
            .zip(parameters.iter())
            .enumerate()
        {
            if !self.conforms(expected, provided) {
                errors.push(format!(
                    "Type {} expects parameter {} of type {}, but got {}",
                    type_definition.name.id,
                    i,
                    to_string(expected),
                    to_string(provided)
                ));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
