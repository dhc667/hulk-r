use crate::tokens::*;
use super::Expression;

pub struct DestructiveAssignment {
    pub identifier: Identifier,
    pub op: BinaryOperator,
    pub expression: Box<Expression>,
}

impl DestructiveAssignment {
    pub fn new(identifier: Identifier, op: BinaryOperator, rhs: Expression) -> Self {
        DestructiveAssignment {
            identifier,
            op,
            expression: Box::new(rhs),
        }
    }
}
