
use std::vec;

use super::super::Expression;
use super::assignment::Assignment;
use crate::tokens::*;
use crate::visitors::ExpressionVisitor;
use crate::visitors::visitable_expression::VisitableExpression;


pub struct LetIn {
    pub let_token: Keyword,
    pub assignment: Box<Assignment>,
    pub in_token: Keyword,
    pub body: Box<Expression>,
}

impl LetIn {
    pub fn new(
        let_token: Keyword,
        assignments: Vec<Assignment>,
        in_token: Keyword,
        expression: Expression,
    ) -> Self {
        let mut iter = assignments.into_iter();
        let first = iter
            .next()
            .expect("Assignment list must contain at least one assignment");

        LetIn {
            let_token,
            assignment: Box::new(first),
            in_token,
            body: LetIn::build_let_in_expression(let_token, iter, in_token, expression),
        }
    }

    fn build_let_in_expression(
        let_token: Keyword,
        mut assignments: vec::IntoIter<Assignment>,
        in_token: Keyword,
        expression: Expression,
    ) -> Box<Expression> {
        let current_assignment = assignments.next();

        match current_assignment {
            None => Box::new(expression),

            Some(assignment) => Box::new(Expression::LetIn(LetIn {
                let_token,
                assignment: Box::new(assignment),
                in_token,
                body: LetIn::build_let_in_expression(let_token, assignments, in_token, expression),
            })),
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for LetIn {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_let_in(self)
    }
}

