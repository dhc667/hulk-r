use std::hash::Hash;

use crate::{parser::StateId, Symbol};

pub struct State {
    pub id: StateId,
    pub symbol: Symbol,
}
