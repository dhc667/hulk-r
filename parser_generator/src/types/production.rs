use crate::{NonTerminalId, SymbolId};

pub type ProductionCompute<R> = Box<dyn Fn(Vec<R>) -> R>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy, PartialOrd, Ord)]
pub struct ProductionId(usize);

impl ProductionId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub struct Production {
    pub lhs: NonTerminalId,
    pub rhs: Vec<SymbolId>,
}

impl Production {
    pub fn new(lhs: NonTerminalId, rhs: Vec<SymbolId>) -> Self {
        Self { lhs, rhs }
    }
}
