use crate::{Block, Expression, ExpressionVisitor, Keyword, VisitableExpression};

#[derive(Debug)]
pub struct While {
    pub while_token: Keyword,
    pub condition: Box<Expression>,
    pub body: Box<Block>,
}

impl While {
    pub fn new(while_token: Keyword, condition: Expression, body: Block) -> Self {
        While {
            while_token,
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for While {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_while(self)
    }
}
