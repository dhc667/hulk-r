use crate::{visitors::visitable::Visitable, Visitor};

use super::ExpressionList;

pub struct Program {
    pub expression_list: ExpressionList
}

impl Program {
    pub fn new(expression_list: ExpressionList) -> Self {
        Program {
            expression_list
        }
    }
}

impl<T: Visitor<R>, R> Visitable<T, R> for Program {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_program(self)
    }
}
