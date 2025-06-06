use std::collections::{HashMap, HashSet};

use crate::regex_engine::regex_ast::symbol::Symbol;

pub struct NFA {
    pub q0: usize,
    pub qf: usize,
    pub d: HashMap<(usize, Symbol), HashSet<usize>>,
}

impl NFA {
    pub fn new(q0: usize, qf: usize, d: HashMap<(usize, Symbol), HashSet<usize>>) -> Self {
        NFA { q0, qf, d }
    }
}
