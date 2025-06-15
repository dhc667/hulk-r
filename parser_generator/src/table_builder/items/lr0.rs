use std::{collections::BTreeSet, fmt::Debug, hash::Hash};

use crate::{ProductionId, SymbolId, table_builder::TableBuilder};

/// # Description
///
/// LR0 items are defined as a production with a dot in a given index of its
/// right hand side, for example `E -> E . + T`, this would be represented
/// as `LR0Item { production_id: id, dot_position: 1 }`
#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
pub struct LR0Item {
    production_id: ProductionId,
    /// This indicates the array position i such that the dot is to the left of i
    dot_position: usize,
}

impl LR0Item {
    pub fn new(production_id: ProductionId, dot_position: usize) -> Self {
        Self {
            production_id,
            dot_position,
        }
    }

    pub fn production_id(&self) -> &ProductionId {
        &self.production_id
    }

    pub fn dot_position(&self) -> usize {
        self.dot_position
    }
}

impl<'a> TableBuilder<'a> {
    /// returns `None` if the dot is at the end of the LR0 item
    ///
    /// If called on item `E -> E . + T` for example, it would return the id of `+`
    pub(crate) fn symbol_right_of_dot(&self, lr0_item: &LR0Item) -> Option<SymbolId> {
        let production = self.productions.get(&lr0_item.production_id).unwrap();
        if lr0_item.dot_position < production.rhs.len() {
            let symbol = production.rhs[lr0_item.dot_position];

            Some(symbol)
        } else {
            None
        }
    }

    /// returns an LR0 item of the same production as `lr0_item` if the dot is
    /// not at the end, or the unchanged LR0 item if it is
    ///
    /// for `E -> E . + T` it would return `E -> E + . T`
    /// for `E -> E .` it would return `E -> E .`
    pub(crate) fn increased_dot_position(&self, mut lr0_item: LR0Item) -> Result<LR0Item, LR0Item> {
        if self.symbol_right_of_dot(&lr0_item).is_none() {
            Err(lr0_item)
        } else {
            lr0_item.dot_position += 1;
            Ok(lr0_item)
        }
    }
}

pub type LR0ItemSet = BTreeSet<LR0Item>;

impl<'a> TableBuilder<'a> {
    pub(in crate::table_builder) fn kernel(&self, lr0_item_set: LR0ItemSet) -> LR0ItemSet {
        LR0ItemSet::from_iter(
            lr0_item_set.into_iter().filter(|item| {
                item.dot_position > 0 || self.first_production_id == item.production_id
            }),
        )
    }
}
