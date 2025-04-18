use super::Expression;
use crate::{tokens::*, visitors::{visitable::Visitable, Visitor}};

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

impl<T: Visitor<R>, R> Visitable<T, R> for DestructiveAssignment {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_destructive_assignment(self)
    }
}
