use super::super::atoms::Atom;
use crate::tokens::UnaryOperator;

pub struct UnOp {
    pub op: UnaryOperator,
    pub rhs: Box<Atom>,
}

impl UnOp {
    pub fn new(op: UnaryOperator, rhs: Atom) -> Self {
        UnOp {
            op,
            rhs: Box::new(rhs),
        }
    }
}
