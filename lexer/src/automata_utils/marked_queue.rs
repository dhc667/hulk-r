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

    pub fn iter(&self) -> std::collections::hash_set::Iter<T> {
        self.marked.iter()
    }
}

impl<'a, T> IntoIterator for &'a MarkedQueue<T>
where
    T: Eq + Hash + Clone,
{
    type Item = &'a T;
    type IntoIter = std::collections::hash_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.marked.iter()
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
