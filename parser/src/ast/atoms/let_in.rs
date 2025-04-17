use crate::tokens::*;
use super::super::Expression;
use super::Atom;

pub struct Assignment {
    pub identifier: Identifier,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
}

impl Assignment {
    pub fn new(identifier: Identifier, op: BinaryOperator, rhs: Expression) -> Self {
        Assignment {
            identifier,
            op,
            rhs: Box::new(rhs),
        }
    }
}





pub struct LetExpression {
    pub let_token: Keyword,
    pub assignments: Vec<Assignment>,
    pub in_token: Keyword,
    pub body: Box<Atom>,
}

impl LetExpression {
    pub fn new(
        let_token: Keyword,
        assignments: Vec<Assignment>,
        in_token: Keyword,
        expression: Atom,
    ) -> Self {
        LetExpression {
            let_token,
            assignments,
            in_token,
            body: Box::new(expression),
        }
    }
}

