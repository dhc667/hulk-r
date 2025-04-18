use super::super::Expression;
use super::Atom;
use crate::tokens::*;
use crate::visitors::Visitor;
use crate::visitors::visitable::Visitable;

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

pub struct LetIn {
    pub let_token: Keyword,
    pub assignments: Vec<Assignment>,
    pub in_token: Keyword,
    pub body: Box<Atom>,
}

impl LetIn {
    pub fn new(
        let_token: Keyword,
        assignments: Vec<Assignment>,
        in_token: Keyword,
        expression: Atom,
    ) -> Self {
        LetIn {
            let_token,
            assignments,
            in_token,
            body: Box::new(expression),
        }
    }
}

impl<T: Visitor<R>, R> Visitable<T, R> for LetIn {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_let_in(self)
    }
}

impl<T: Visitor<R>, R> Visitable<T, R> for Assignment {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_assignment(self)
    }
}
