use super::dfs::{DfsVisitable, dfs};

/// # Description
/// Implements the Lowest Common Ancestor (LCA) algorithm using a Range Minimum Query (RMQ) approach on the Euler tour of a tree.
/// The LCA structure stores the height of each node, the Euler tour, the first occurrence of each node in the tour,
/// and a sparse table for efficient LCA queries.
/// # Example
/// ```
/// use semantic_analyzer::graph_utils::lca::LCA;
/// let adj = vec![
///     vec![1, 2],     // Node 0 is connected to nodes 1 and 2
///     vec![0, 3, 4],  // Node 1 is connected to nodes 0, 3, and 4
///     vec![0],        // Node 2 is connected to node 0
///     vec![1],        // Node 3 is connected to node 1
///     vec![1],        // Node 4 is connected to node 1
/// ];
/// let lca = LCA::new(&adj, 0);
/// let lca_node = lca.get_lca(3, 4); // Should return 1, the LCA of nodes 3 and 4
/// use std::vec;
/// assert_eq!(lca_node, 1);
/// ```
/// # Note
/// This implementation assumes that the input graph is a tree (i.e., it is connected and acyclic).
///     
/// # Complexity
///  - **Time Complexity**: O(1) for each LCA query and O(n log n) for preprocessing.
///  - **Space Complexity**: O(n log n) for the sparse table and O(n) for the Euler tour and height arrays.
pub struct LCA {
    height: Vec<usize>,
    euler: Vec<usize>,
    first: Vec<usize>,
    sparse_table: Vec<Vec<usize>>,
}

impl LCA {
    /// # Description
    /// Creates a new LCA instance from an adjacency list representation of a tree.
    /// # Arguments
    /// * `adj`: A vector of vectors representing the adjacency list of the tree.
    /// * `root`: The index of the root node in the tree.
    /// # Returns
    /// A new instance of `LCA` initialized with the given adjacency list and root node.
    /// # Note
    /// This function asumes that the input graph is a tree (i.e., it is connected and acyclic) and that the root node is valid.
    pub fn new(adj: &Vec<Vec<usize>>, root: usize) -> Self {
        let n = adj.len();
        let mut lca = LCA {
            height: vec![0; n],
            euler: Vec::with_capacity(n * 2),
            first: vec![0; n],
            sparse_table: Vec::new(),
        };

        dfs(adj, root, &mut lca);
        lca.build_sparse_table();
        lca
    }

    fn build_sparse_table(&mut self) {
        let m = self.euler.len();
        let log = (m as f64).log2() as usize + 1;
        self.sparse_table = vec![vec![0; m]; log];

        for i in 0..m {
            self.sparse_table[0][i] = i;
        }

        for k in 1..log {
            for i in 0..=m - (1 << k) {
                let left = self.sparse_table[k - 1][i];
                let right = self.sparse_table[k - 1][i + (1 << (k - 1))];
                self.sparse_table[k][i] =
                    if self.height[self.euler[left]] < self.height[self.euler[right]] {
                        left
                    } else {
                        right
                    };
            }
        }
    }

    /// # Description
    /// Finds the Lowest Common Ancestor (LCA) of two nodes in a tree.
    /// # Arguments
    /// * `u`: The first node.
    /// * `v`: The second node.
    /// # Returns
    /// The index of the LCA node.
    /// # Note
    /// This function assumes that both `u` and `v` are valid nodes in the tree.
    pub fn get_lca(&self, u: usize, v: usize) -> usize {
        let mut l = self.first[u];
        let mut r = self.first[v];
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        let len = r - l + 1;
        let k = (len as f64).log2() as usize;
        let left = self.sparse_table[k][l];
        let right = self.sparse_table[k][r + 1 - (1 << k)];
        if self.height[self.euler[left]] < self.height[self.euler[right]] {
            self.euler[left]
        } else {
            self.euler[right]
        }
    }
}

impl DfsVisitable for LCA {
    fn before_visit(&mut self, node: usize, h: usize) {
        self.height[node] = h;
        self.first[node] = self.euler.len();
        self.euler.push(node);
    }

    fn after_visit_child(&mut self, node: usize, _h: usize) {
        self.euler.push(node);
    }
}
