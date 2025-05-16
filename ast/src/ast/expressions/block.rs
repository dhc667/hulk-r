use super::super::Expression;
use crate::{
    tokens::GroupingOperator,
    visitors::{Visitor, visitable::Visitable},
};

pub struct ExpressionList {
    pub expressions: Vec<Expression>,
    pub multiple_semicolon_terminated: bool,
}

impl ExpressionList {
    pub fn new(expressions: Vec<Expression>, multiple_semicolon_terminated: bool) -> Self {
        ExpressionList {
            expressions,
            multiple_semicolon_terminated,
        }
    }
}

impl<T: Visitor<R>, R> Visitable<T, R> for ExpressionList {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_expression_list(self)
    }
}

pub struct Block {
    pub open_brace: GroupingOperator,
    pub close_brace: GroupingOperator,
    pub expression_list: ExpressionList,
}

impl Block {
    pub fn new(
        open_brace: GroupingOperator,
        expression_list: ExpressionList,
        close_brace: GroupingOperator,
    ) -> Self {
        Block {
            open_brace,
            close_brace,
            expression_list,
        }
    }
}

impl<T: Visitor<R>, R> Visitable<T, R> for Block {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_block(self)
    }
}
