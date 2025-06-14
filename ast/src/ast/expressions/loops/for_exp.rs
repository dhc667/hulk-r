use crate::{Block, Expression, ExpressionVisitor, Identifier, Keyword, VisitableExpression};

#[derive(Debug)]
pub struct For {
    pub for_token: Keyword,
    pub element: Identifier,
    pub in_token: Keyword,
    pub iterable: Box<Expression>,
    pub body: Block,
}

impl For {
    pub fn new(
        for_token: Keyword,
        element: Identifier,
        in_token: Keyword,
        iterable: Expression,
        body: Block,
    ) -> Self {
        Self {
            for_token,
            element,
            in_token,
            iterable: Box::new(iterable),
            body,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for For {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_for(self)
    }
}
