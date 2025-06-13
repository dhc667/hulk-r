use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Index,
};

/// # Description
/// A marked queue. It behabes as a regular LIFO queue,
/// except it keeps track of the elements that where previously added. It also computes indexes,
/// which are equal to the order of first add
///
/// # Methods
/// - `add_unmarked(item: T)`: it adds an item to the queue if it has not been added before.
/// Returns `true` if it was succesfully added, `false` if it has been added before
///
/// - `pop_unmarked`: returns the first item in the queue, or None if the queue is empty.
/// - `contains`: returns true if the item has been added before, false otherwise.
/// - `iter`: returns an iterator over the items in the queue.
///
/// # Example
/// ```
/// use lexer::automata_utils::marked_queue::MarkedQueue;
/// let mut queue = MarkedQueue::new();
/// queue.add_unmarked(1);
/// queue.add_unmarked(2);
/// assert!(queue.contains(&1));
/// assert_eq!(queue.pop_unmarked().unwrap(), 2);
/// assert_eq!(queue[&1], 0);
/// assert!(queue.contains(&1))
///

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
    /// # Description
    /// Creates a new empty `MarkedQueue`.
    pub fn new() -> Self {
        MarkedQueue {
            unmarked: Vec::new(),
            marked: HashSet::new(),
            index: HashMap::new(),
        }
    }

    /// # Description
    /// Adds an item to the queue if it has not been added before.
    /// Returns `true` if it was successfully added, `false` if it has been added before.
    pub fn add_unmarked(&mut self, item: T) -> bool {
        if self.marked.contains(&item) {
            return false;
        }
        self.marked.insert(item.clone());
        self.index.insert(item.clone(), self.index.len());
        self.unmarked.push(item);
        true
    }

    /// # Description
    /// Returns the first item in the queue, or `None` if the queue is empty.
    pub fn pop_unmarked(&mut self) -> Option<T> {
        self.unmarked.pop()
    }

    /// # Description
    /// Returns `true` if the item has been added before, `false` otherwise.
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

    /// # Description
    /// Returns the order of first add of the item.
    fn index(&self, item: &T) -> &Self::Output {
        &self.index[item]
    }
}
