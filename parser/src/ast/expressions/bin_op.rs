use super::Expression;
use crate::tokens::*;

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
