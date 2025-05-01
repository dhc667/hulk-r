use super::Expression;
use crate::{
    tokens::*,
    visitors::{Visitor, visitable::Visitable},
};

pub struct BinOp {
    pub lhs: Box<Expression>,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
}

impl BinOp {
    pub fn new(lhs: Expression, op: BinaryOperator, rhs: Expression) -> Self {
        BinOp {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl<T: Visitor<R>, R> Visitable<T, R> for BinOp {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_bin_op(self)
    }
}
