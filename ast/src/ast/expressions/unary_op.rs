use crate::{
    tokens::UnaryOperator,
    visitors::{ExpressionVisitor, visitable_expression::VisitableExpression},
};

use super::Expression;

pub struct UnOp {
    pub op: UnaryOperator,
    pub rhs: Box<Expression>,
}

impl UnOp {
    pub fn new(op: UnaryOperator, rhs: Expression) -> Self {
        UnOp {
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for UnOp {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_un_op(self)
    }
}
