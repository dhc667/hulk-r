use crate::{ExpressionVisitor, Keyword, VisitableExpression};

use super::{Expression, FunctionCall};

pub struct NewExpr {
    pub new_token: Keyword,
    pub type_name: String,
    pub arguments: Vec<Expression>,
}

impl NewExpr {
    pub fn new(new_token: Keyword, type_name: String, arguments: Vec<Expression>) -> Self {
        Self {
            new_token,
            type_name,
            arguments,
        }
    }

    pub fn from_function_call(new_token: Keyword, function_call: FunctionCall) -> Self {
        Self {
            new_token,
            type_name: function_call.identifier.id,
            arguments: function_call.arguments,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for NewExpr {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_new_expr(self)
    }
}
