use std::{collections::{HashMap, HashSet}, fmt::Debug, hash::Hash};

use crate::{parser::ProductionId, parser_generator::ParserGenerator, symbol::TerminalId, SymbolId};

#[derive(PartialEq, Eq, Debug)]
pub struct LR1Item {
    pub core: LR0Item,
    pub follow: HashSet<TerminalId>,
}

impl LR1Item {
    /// fails if the follow set is empty
    pub fn build(core: LR0Item, follow: HashSet<TerminalId>) -> Result<LR1Item, ()> {
        if follow.is_empty() {
            return Err(());
        }

        Ok(Self { core, follow })
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

#[derive(Debug, Clone)]
pub struct LR1ItemSetBuilder {
    pub items: HashMap<LR0Item, HashSet<TerminalId>>,
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
        //  ERROR: Breaking invariant
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

    pub fn insert_to_core_if_exists(&mut self, core: &LR0Item, symbols: &HashSet<TerminalId>) -> bool {
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

    pub fn merge(&mut self, other: Self) -> bool {
        let mut changed = false;
        for (core, follow) in other.items.into_iter() {
            changed |= self.insert(LR1Item::build(core, follow).unwrap())
        }

        changed
    }

    pub fn to_hash_set(self) -> HashSet<LR1Item> {
        self.items
            .into_iter()
            .map(|(core, follow)| LR1Item::build(core, follow).unwrap())
            .collect()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
pub struct LR0Item {
    pub production_id: ProductionId,
    /// This indicates the array position i such that the dot is to the left of i
    pub dot_position: usize,
}

impl LR0Item {
    pub fn new(production_id: ProductionId, dot_position: usize) -> Self {
        Self {
            production_id,
            dot_position,
        }
    }
}

impl<TokenType: Eq + Hash + Copy + Debug, R> ParserGenerator<TokenType, R> {
    pub(crate) fn symbol_right_of_dot(&self, lr0_item: &LR0Item) -> Option<SymbolId> {
        let production = self.productions.get(&lr0_item.production_id).unwrap();
        if lr0_item.dot_position < production.rhs.len() {
            Some(production.rhs[lr0_item.dot_position])
        } else {
            None
        }
    }

    pub(crate) fn increased_dot_position(&self, mut lr0_item: LR0Item) -> Result<LR0Item, ()> {
        if self.symbol_right_of_dot(&lr0_item).is_none() {
            Err(())
        } else {
            lr0_item.dot_position += 1;
            Ok(lr0_item)
        }
    }
}
