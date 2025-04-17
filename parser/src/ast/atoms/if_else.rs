use super::super::Expression;
use super::Atom;
use crate::tokens::Keyword;

pub struct IfElse {
    pub if_token: Keyword,
    pub condition: Box<Expression>,
    pub then_expression: Box<Atom>,
    pub else_token: Keyword,
    pub else_expression: Box<Atom>,
}

impl IfElse {
    pub fn new(
        if_token: Keyword,
        condition: Expression,
        then_expression: Atom,
        else_token: Keyword,
        else_expression: Atom,
    ) -> Self {
        IfElse {
            if_token,
            condition: Box::new(condition),
            then_expression: Box::new(then_expression),
            else_token,
            else_expression: Box::new(else_expression),
        }
    }
}
