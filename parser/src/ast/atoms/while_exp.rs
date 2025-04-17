use super::super::Expression;
use super::Atom;
use crate::tokens::Keyword;

pub struct While {
    pub while_token: Keyword,
    pub condition: Box<Expression>,
    pub body: Box<Atom>,
}

impl While {
    pub fn new(while_token: Keyword, condition: Expression, body: Atom) -> Self {
        While {
            while_token,
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}
