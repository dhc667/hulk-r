use crate::symbol::{NonTerminalId, SymbolId};

pub struct Production<R> {
    pub lhs: NonTerminalId,
    pub rhs: Vec<SymbolId>,
    pub compute: Box<dyn Fn(Vec<R>) -> R>,
}

impl<R> std::fmt::Debug for Production<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Production").field("lhs", &self.lhs).field("rhs", &self.rhs).finish()
    }
}

impl<R> Production<R> {
    pub fn new(
        lhs: NonTerminalId,
        rhs: Vec<SymbolId>,
        compute: impl Fn(Vec<R>) -> R + 'static,
    ) -> Self {
        Self {
            lhs,
            rhs,
            compute: Box::new(compute),
        }
    }
}
