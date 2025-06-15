use std::collections::HashMap;

use ast::{
    Definition,
    typing::{BuiltInType, Type, TypeAnnotation},
};

use crate::graph_utils::{TopologicalSort, parent_map_to_adj};

pub fn sort_definitions(
    type_hierarchy: &HashMap<String, TypeAnnotation>,
    definition_list: &mut Vec<Definition>,
) {
    let type_ids: HashMap<_, _> = type_hierarchy.keys().cloned().zip((0 as usize)..).collect();
    let adj = parent_map_to_adj(type_hierarchy, &type_ids);
    let object_name = Type::BuiltIn(BuiltInType::Object).to_string();
    let root = type_ids[&object_name];

    let mut topological_sort = TopologicalSort::new();
    let mut sorted_ids = topological_sort.get_sorted(&adj, root);
    sorted_ids.reverse();
    let type_index: HashMap<_, _> = sorted_ids
        .iter()
        .enumerate()
        .map(|(index, &id)| (id, index))
        .collect();

    definition_list.sort_by(|a, b| match (a, b) {
        (Definition::TypeDef(a), Definition::TypeDef(b)) => {
            let a_id = type_ids[&a.name.id];
            let b_id = type_ids[&b.name.id];
            let a_index = type_index[&a_id];
            let b_index = type_index[&b_id];
            a_index.cmp(&b_index)
        }
        (Definition::TypeDef(_), _) => std::cmp::Ordering::Less,
        (_, Definition::TypeDef(_)) => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
    });
}
