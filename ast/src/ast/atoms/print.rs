use super::super::Expression;
use crate::{
    tokens::Keyword,
    visitors::{Visitor, visitable::Visitable},
};

pub struct Print {
    pub print_token: Keyword,
    pub expression: Box<Expression>,
}

impl<T: Visitor<R>, R> Visitable<T, R> for Print {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_print(self)
    }
}
