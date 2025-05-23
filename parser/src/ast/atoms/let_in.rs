use std::vec;

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
    pub assignment: Box<Assignment>,
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
        let mut iter = assignments.into_iter();
        let first = iter.next().expect("Assignment list must contain at least one assignment");

        LetIn{
            let_token,
            assignment: Box::new(first),
            in_token,
            body: LetIn::build_let_in_expression(let_token, iter, in_token, expression),
        }
    }

    fn build_let_in_expression(let_token: Keyword, mut assignments: vec::IntoIter<Assignment>, in_token: Keyword, expression: Atom) -> Box<Atom> {
        let current_assignment = assignments.next();

        match current_assignment {
            None => Box::new(expression),

            Some(assignment) => {
                Box::new(Atom::LetIn(LetIn{
                    let_token,
                    assignment: Box::new(assignment),
                    in_token,
                    body: LetIn::build_let_in_expression(let_token, assignments, in_token, expression)
                }))
            }
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
