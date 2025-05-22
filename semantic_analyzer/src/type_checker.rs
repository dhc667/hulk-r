use std::collections::HashMap;

use ast::typing::{Type, TypeAnnotation, to_string};
use generator::context::Context;

use crate::{DefinedTypeInfo, FuncInfo, TypeInfo, lca::LCA};

pub struct TypeChecker<'a> {
    type_ids: HashMap<String, usize>,
    type_names: Vec<String>,
    type_definitions: &'a Context<TypeInfo>,
    lca: LCA,
}

impl<'a> TypeChecker<'a> {
    pub fn new(
        type_hierarchy: &HashMap<String, TypeAnnotation>,
        type_definitions: &'a Context<TypeInfo>,
    ) -> Self {
        let mut type_ids = HashMap::new();
        let mut type_names = Vec::new();
        for (i, type_name) in type_hierarchy.keys().enumerate() {
            type_ids.insert(type_name.clone(), i);
            type_names.push(type_name.clone());
        }
        let adj = Self::build_adjacency_list(&type_hierarchy, &type_ids);
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

    fn build_adjacency_list(
        type_hierarchy: &HashMap<String, TypeAnnotation>,
        type_ids: &HashMap<String, usize>,
    ) -> Vec<Vec<usize>> {
        let mut adj = vec![Vec::new(); type_hierarchy.len()];
        for type_name in type_hierarchy.keys() {
            let id = type_ids[type_name];
            let parent = &type_hierarchy[type_name];
            if parent.is_none() {
                continue;
            }
            if let Some(parent_id) = type_ids.get(&parent.clone().unwrap().to_string()) {
                adj[*parent_id].push(id);
                adj[id].push(*parent_id);
            }
        }
        adj
    }

    /// # Description
    /// Converts a type to its id in the type tree graph
    /// Note: it asumes that ty is defined, will panic if it is not
    fn type_to_id(&self, ty: &Type) -> usize {
        let type_name = ty.to_string();
        let id = self.type_ids.get(&type_name);
        *id.unwrap()
    }

    pub fn is_subtype(&self, a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
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
                    if let Some(common_type) = self.type_definitions.get_value(common_name) {
                        return match common_type {
                            TypeInfo::Defined(ty) => Some(Type::Defined(ty.name.clone())),
                            TypeInfo::BuiltIn(ty) => Some(Type::BuiltIn(ty.clone())),
                        };
                    }
                }
                None
            }
        }
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
                "Function {} {} has {} parameters, but {} were provided",
                fn_info.name,
                functor,
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
            if !self.is_subtype(expected, provided) {
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
            if !self.is_subtype(expected, provided) {
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
