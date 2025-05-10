use crate::{tokens, ExpressionVisitor, VisitableExpression};

use super::Expression;

pub struct ListLiteral {
    pub left_bracket: tokens::GroupingOperator,
    pub right_bracket: tokens::GroupingOperator,
    pub elements: Vec<Expression>,
}
impl ListLiteral {
    pub fn new(
        left_bracket: tokens::GroupingOperator,
        right_bracket: tokens::GroupingOperator,
        elements: Vec<Expression>,
    ) -> Self {
        Self {
            left_bracket,
            right_bracket,
            elements,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for ListLiteral {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_list_literal(self)
    }
}
