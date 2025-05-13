use crate::{Expression, ExpressionVisitor, Keyword, VisitableExpression};

pub struct ReturnStatement {
    pub return_token: Keyword,
    pub expression: Expression,
}

impl ReturnStatement {
    pub fn new(return_token: Keyword, expression: Expression) -> Self {
        Self {
            return_token,
            expression,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for ReturnStatement {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_return_statement(self)
    }
}
