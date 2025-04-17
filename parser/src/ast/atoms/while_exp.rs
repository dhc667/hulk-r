use crate::tokens::Keyword;
use super::super::Expression;
use super::Atom;


pub struct WhileExpression {
    pub while_token: Keyword,
    pub condition: Box<Expression>,
    pub body: Box<Atom>,
}

impl WhileExpression {
    pub fn new(while_token: Keyword, condition: Expression, body: Atom) -> Self {
        WhileExpression {
            while_token,
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}
