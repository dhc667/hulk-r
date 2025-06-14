use crate::parser::{ProductionId, StateId};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    Shift(StateId),
    Reduce(ProductionId),
    Accept,
}
