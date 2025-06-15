use crate::graph_utils::{DfsVisitable, dfs};

pub struct TopologicalSort {
    stack: Vec<usize>,
}

impl TopologicalSort {
    pub fn new() -> Self {
        TopologicalSort { stack: Vec::new() }
    }

    pub fn get_sorted(&mut self, adj: &Vec<Vec<usize>>, root: usize) -> Vec<usize> {
        dfs(adj, root, self);
        self.stack.clone()
    }
}

impl DfsVisitable for TopologicalSort {
    fn after_visit(&mut self, node: usize, _h: usize) {
        self.stack.push(node);
    }
}
