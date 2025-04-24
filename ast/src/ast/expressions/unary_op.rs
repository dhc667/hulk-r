use super::super::atoms::Atom;
use crate::{tokens::UnaryOperator, visitors::{visitable::Visitable, Visitor}};

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

impl<T: Visitor<R>, R> Visitable<T, R> for UnOp {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_un_op(self)
    }
}
