use std::collections::{HashMap, HashSet};

use crate::{TerminalId, table_builder::items::LR0Item};

/// # Description
///
/// LR1 items are defined as an `LR0 item` that serves as the `core`, and a
/// non-empty set of terminals, which are the specific terminals that can
/// appear following it in a valid right sentential form
#[derive(PartialEq, Eq, Debug)]
pub struct LR1Item {
    core: LR0Item,
    follow: HashSet<TerminalId>,
}

impl LR1Item {
    /// fails if the follow set is empty
    pub fn build(core: LR0Item, follow: HashSet<TerminalId>) -> Result<LR1Item, ()> {
        if follow.is_empty() {
            return Err(());
        }

        Ok(Self { core, follow })
    }

    pub fn core(&self) -> &LR0Item {
        &self.core
    }

    pub fn follow(&self) -> &HashSet<TerminalId> {
        &self.follow
    }
}

impl std::hash::Hash for LR1Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.core.hash(state);
        self.follow
            .iter()
            .map(|elem| *elem)
            .collect::<Vec<TerminalId>>()
            .hash(state);
    }
}

/// Used to efficiently build `LR1` item sets, it avoids the non-empty
/// follow set restriction of `LR1` items
#[derive(Debug, Clone)]
pub struct LR1ItemSetBuilder {
    items: HashMap<LR0Item, HashSet<TerminalId>>,
}

impl From<HashSet<LR1Item>> for LR1ItemSetBuilder {
    fn from(value: HashSet<LR1Item>) -> Self {
        Self {
            items: value
                .into_iter()
                .map(|elem| (elem.core, elem.follow))
                .collect(),
        }
    }
}

impl From<HashSet<LR0Item>> for LR1ItemSetBuilder {
    fn from(value: HashSet<LR0Item>) -> Self {
        Self {
            items: value
                .into_iter()
                .map(|elem| (elem, HashSet::new()))
                .collect(),
        }
    }
}

impl LR1ItemSetBuilder {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Inserts the follow symbols in `item` into the core associated to it
    /// in the item set, creates a new core assignment if it didn't exist
    ///
    /// # Returns
    ///
    /// - true: if a new core was added or the existing core did not have any
    /// of the follow symbols in `item`
    /// - false: otherwise
    pub fn insert(&mut self, item: LR1Item) -> bool {
        let LR1Item { core, follow } = item;
        let mut changed = false;

        if self.items.get(&core).is_none() {
            self.items.insert(core, HashSet::new());
        }
        let target_item = self.items.get_mut(&core).unwrap();

        for elem in follow {
            changed |= target_item.insert(elem)
        }

        changed
    }

    /// Assigns follow `symbols` to the to the specified `core` if it already
    /// appears in the set builder, if it does not, it does nothing
    ///
    /// # Returns
    /// - true: if the given core existed and did not have any of the sent
    /// symbols
    /// - false: otherwise
    pub fn insert_to_core_if_exists(
        &mut self,
        core: &LR0Item,
        symbols: &HashSet<TerminalId>,
    ) -> bool {
        if symbols.is_empty() {
            return false;
        }

        let mut changed = false;

        if !self.items.contains_key(&core) {
            return false;
        }

        let set = self.items.get_mut(&core).unwrap();

        for elem in symbols {
            changed |= set.insert(*elem);
        }

        changed
    }

    /// Merges with another item set builder `other`
    ///
    /// # Returns
    /// - true: if the merge resulted in a change in the original instance
    /// - false: otherwise
    pub fn merge(&mut self, other: Self) -> bool {
        let mut changed = false;
        for (core, follow) in other.items.into_iter() {
            changed |= self.insert(LR1Item::build(core, follow).unwrap())
        }

        changed
    }

    /// Converts the `LR1ItemSetBuilder` into a hash set of `LR1` items,
    ///
    /// # Returns
    /// - Err(self) if any of the item cores has an empty follow set associated
    /// to it, it fails and returns the same instance
    /// - Ok(_) otherwise
    pub fn to_hash_set(self) -> Result<HashSet<LR1Item>, Self> {
        let empty_set_exists = self.items.iter().any(|(_, set)| set.is_empty());

        if empty_set_exists {
            Err(self)
        } else {
            Ok(self
                .items
                .into_iter()
                .map(|(core, follow)| LR1Item::build(core, follow).unwrap())
                .collect())
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&LR0Item, &HashSet<TerminalId>)> {
        self.items.iter()
    }
}
