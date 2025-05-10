use crate::{ExpressionVisitor, GroupingOperator, VisitableExpression};

use super::Expression;

pub struct ListIndexing {
    pub list: Box<Expression>,
    pub open_brace: GroupingOperator,
    pub close_brace: GroupingOperator,
    pub index: Box<Expression>,
}

impl ListIndexing {
    pub fn new(
        list: Expression,
        open_brace: GroupingOperator,
        close_brace: GroupingOperator,
        index: Expression,
    ) -> Self {
        Self {
            list: Box::new(list),
            open_brace,
            close_brace,
            index: Box::new(index),
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for ListIndexing {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visist_list_indexing(self)
    }
}
