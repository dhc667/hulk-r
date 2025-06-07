use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Index,
};

pub struct MarkedQueue<T>
where
    T: Eq + Hash + Clone,
{
    unmarked: Vec<T>,
    marked: HashSet<T>,
    index: HashMap<T, usize>,
}

impl<T> MarkedQueue<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        MarkedQueue {
            unmarked: Vec::new(),
            marked: HashSet::new(),
            index: HashMap::new(),
        }
    }

    pub fn add_unmarked(&mut self, item: T) -> bool {
        if self.marked.contains(&item) {
            return false;
        }
        self.marked.insert(item.clone());
        self.index.insert(item.clone(), self.index.len());
        self.unmarked.push(item);
        true
    }

    pub fn pop_unmarked(&mut self) -> Option<T> {
        self.unmarked.pop()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.marked.contains(item)
    }
}

impl<T> Index<&T> for MarkedQueue<T>
where
    T: Eq + Hash + Clone,
{
    type Output = usize;

    fn index(&self, item: &T) -> &Self::Output {
        &self.index[item]
    }
}
