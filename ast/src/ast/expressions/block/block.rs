use crate::{ExpressionVisitor, GroupingOperator, VisitableExpression};

use super::BlockBodyItem;

pub struct Block {
    pub open_brace: GroupingOperator,
    pub close_brace: GroupingOperator,
    pub body_items: Vec<BlockBodyItem>,
    pub multiple_semicolon_terminated: bool,
}

impl Block {
    pub fn new(
        open_brace: GroupingOperator,
        close_brace: GroupingOperator,
        body_items: Vec<BlockBodyItem>,
        multiple_semicolon_terminated: bool,
    ) -> Self {
        Self {
            open_brace,
            close_brace,
            body_items,
            multiple_semicolon_terminated,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for Block {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_block(self)
    }
}
