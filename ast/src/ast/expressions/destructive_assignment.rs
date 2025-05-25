use super::Expression;
use crate::{
    tokens::*,
    visitors::{ExpressionVisitor, visitable_expression::VisitableExpression},
};

pub struct DestructiveAssignment {
    pub lhs: Box<Expression>,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
}

impl DestructiveAssignment {
    pub fn new(lhs: Expression, op: BinaryOperator, rhs: Expression) -> Self {
        DestructiveAssignment {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for DestructiveAssignment {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_destructive_assignment(self)
    }
}
