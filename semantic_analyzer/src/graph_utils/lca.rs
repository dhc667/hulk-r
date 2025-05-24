use super::dfs::{DfsVisitable, dfs};

pub struct LCA {
    height: Vec<usize>,
    euler: Vec<usize>,
    first: Vec<usize>,
    sparse_table: Vec<Vec<usize>>,
}

impl LCA {
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
