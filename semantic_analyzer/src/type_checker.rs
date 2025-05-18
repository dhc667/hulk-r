use std::collections::HashMap;

use ast::typing::{Type, TypeAnnotation};
use generator::context::Context;

use crate::lca::LCA;

pub struct TypeChecker<'a> {
    type_ids: HashMap<String, usize>,
    type_names: Vec<String>,
    type_definitions: &'a Context<Type>,
    lca: LCA,
}

impl<'a> TypeChecker<'a> {
    pub fn new(
        type_hierarchy: &HashMap<String, TypeAnnotation>,
        type_definitions: &'a Context<Type>,
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
            (_, None) => return false,
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
                        return Some(common_type.clone());
                    }
                }
                None
            }
        }
    }
}
