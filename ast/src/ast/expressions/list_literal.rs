use crate::{ExpressionVisitor, VisitableExpression, tokens, typing::TypeAnnotation};

use super::Expression;

pub struct ListLiteral {
    pub left_bracket: tokens::GroupingOperator,
    pub right_bracket: tokens::GroupingOperator,
    pub elements: Vec<Expression>,
    pub list_type: TypeAnnotation,
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
            list_type: None,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for ListLiteral {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_list_literal(self)
    }
}
