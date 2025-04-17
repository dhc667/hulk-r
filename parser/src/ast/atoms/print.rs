use crate::tokens::Keyword;
use super::super::Expression;

pub struct PrintExpression {
    pub print_token: Keyword,
    pub expression: Box<Expression>,
}
