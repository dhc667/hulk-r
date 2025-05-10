use crate::{ExpressionVisitor, Identifier, VisitableExpression};

use super::Expression;

pub struct FunctionCall {
    pub identifier: Identifier,
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn new(identifier: Identifier, arguments: Vec<Expression>) -> Self {
        Self {
            identifier,
            arguments,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for FunctionCall {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_function_call(self)
    }
}
