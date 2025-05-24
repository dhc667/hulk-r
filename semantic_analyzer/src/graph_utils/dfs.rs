use std::{collections::HashMap, fmt::Display};

pub fn dfs<V: DfsVisitable>(adj: &Vec<Vec<usize>>, root: usize, visitor: &mut V) {
    let mut visited = vec![false; adj.len()];
    dfs_visit(adj, &mut visited, root, 0, visitor);
}

fn dfs_visit<V: DfsVisitable>(
    adj: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    node: usize,
    h: usize,
    visitor: &mut V,
) {
    visited[node] = true;
    visitor.before_visit(node, h);
    for &to in &adj[node] {
        if !visited[to] {
            dfs_visit(adj, visited, to, h + 1, visitor);
            visitor.after_visit_child(node, h);
        }
    }
}

pub trait DfsVisitable {
    fn before_visit(&mut self, node: usize, h: usize);
    fn after_visit_child(&mut self, node: usize, h: usize);
}

/// # Description
/// Gets the cycle in a tree represented by a parent map.
/// If there is no cycle, returns None.
pub fn get_cycle<T: Display>(tree: &HashMap<String, Option<T>>) -> Option<Vec<String>> {
    let mut visiting: HashMap<String, bool> = HashMap::new();
    let mut tree_path: Vec<String> = Vec::new();
    for (node, _) in tree.iter() {
        let node_result = has_cycles_helper(tree, node, &mut visiting, &mut tree_path);
        match node_result {
            Some(path) => {
                return Some(path);
            }
            _ => {}
        }
    }
    None
}

fn has_cycles_helper<T: Display>(
    tree: &HashMap<String, Option<T>>,
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
    if let Some(parent) = tree.get(node) {
        if let Some(parent_type) = parent {
            let node_result =
                has_cycles_helper(tree, &parent_type.to_string(), visiting, tree_path);
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
