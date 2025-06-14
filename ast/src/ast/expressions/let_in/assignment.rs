use crate::{BinaryOperator, Expression, ExpressionVisitor, Identifier, VisitableExpression};

#[derive(Debug)]
pub struct Assignment {
    pub identifier: Identifier,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
}

impl Assignment {
    pub fn new(identifier: Identifier, op: BinaryOperator, rhs: Expression) -> Self {
        Assignment {
            identifier,
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for Assignment {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_assignment(self)
    }
}
