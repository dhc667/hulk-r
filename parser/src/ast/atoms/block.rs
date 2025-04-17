use super::super::Expression;
use crate::tokens::GroupingOperator;

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

pub struct Block {
    pub open_brace: GroupingOperator,
    pub close_brace: GroupingOperator,
    pub expressions: Vec<Expression>,
    pub multiple_semicolon_terminated: bool,
}

impl Block {
    pub fn new(open_brace: GroupingOperator, expression_list: ExpressionList, close_brace: GroupingOperator) -> Self {
        Block {
            open_brace,
            close_brace,
            expressions: expression_list.expressions,
            multiple_semicolon_terminated: expression_list.multiple_semicolon_terminated,
        }
    }
}
