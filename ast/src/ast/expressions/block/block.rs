use crate::{ExpressionVisitor, GroupingOperator, VisitableExpression};

use super::block_body::BlockBody;

pub struct Block {
    pub open_brace: GroupingOperator,
    pub close_brace: GroupingOperator,
    pub body: BlockBody,
}

impl Block {
    pub fn new(
        open_brace: GroupingOperator,
        expression_list: BlockBody,
        close_brace: GroupingOperator,
    ) -> Self {
        Block {
            open_brace,
            close_brace,
            body: expression_list,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for Block {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_block(self)
    }
}
