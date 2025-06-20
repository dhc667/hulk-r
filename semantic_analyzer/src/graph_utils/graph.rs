use std::{collections::HashMap, fmt::Display};

/// # Description
/// Gets the adjacency list representation of a parent map.
/// The parent map is a mapping from type names to their parent types.
/// # Arguments
/// * `parent_map`: A HashMap where keys are type names (as Strings) and values are optional parent nodes.
/// * `keys_ids`: A HashMap mapping type names to their corresponding indices in the adjacency list.
/// # Returns
/// A vector of vectors, where each inner vector contains the indices of the adjacent for each node.
pub fn parent_map_to_adj<T: Display>(
    parent_map: &HashMap<String, Option<T>>,
    keys_ids: &HashMap<String, usize>,
) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); parent_map.len()];
    for type_name in parent_map.keys() {
        let id = keys_ids[type_name];
        let parent = &parent_map[type_name];
        match parent {
            Some(parent_value) => {
                if let Some(parent_id) = keys_ids.get(&parent_value.to_string()) {
                    adj[*parent_id].push(id);
                    adj[id].push(*parent_id);
                }
            }
            None => continue,
        }
    }
    adj
}
