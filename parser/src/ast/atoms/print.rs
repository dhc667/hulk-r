use super::super::Expression;
use crate::tokens::Keyword;

pub struct Print {
    pub print_token: Keyword,
    pub expression: Box<Expression>,
}
