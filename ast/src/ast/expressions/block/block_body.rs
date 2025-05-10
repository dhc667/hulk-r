
use super::body_item::BlockBodyItem;
use crate::{ExpressionVisitor, VisitableExpression};

pub struct BlockBody {
    pub body_items: Vec<BlockBodyItem>,
    pub multiple_semicolon_terminated: bool,
}

impl BlockBody {
    pub fn new(body_items: Vec<BlockBodyItem>, multiple_semicolon_terminated: bool) -> Self {
        BlockBody {
            body_items,
            multiple_semicolon_terminated,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for BlockBody {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_block_body(self)
    }
}

