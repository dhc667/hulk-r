use crate::{Expression, ExpressionVisitor, VisitableExpression};

use super::return_statement::ReturnStatement;


pub enum BlockBodyItem {
    Expression(Expression),
    ReturnStatement(ReturnStatement)
}

impl BlockBodyItem {
    pub fn as_expression(&self) -> Option<&Expression> {
        if let Self::Expression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_return_statement(&self) -> Option<&ReturnStatement> {
        if let Self::ReturnStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<ReturnStatement> for BlockBodyItem {
    fn from(v: ReturnStatement) -> Self {
        Self::ReturnStatement(v)
    }
}

impl From<Expression> for BlockBodyItem {
    fn from(v: Expression) -> Self {
        Self::Expression(v)
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for BlockBodyItem {
    fn accept(&mut self, visitor: &mut T) -> R {
        match self {
            Self::Expression(e) => e.accept(visitor),
            Self::ReturnStatement(s) => s.accept(visitor)
        }
    }
}
