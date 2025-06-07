use std::fmt::Display;

use crate::regex_engine::regex_ast::regex_exp::RegexExp;

pub enum BinaryOperator {
    Concat,
    Union,
}

pub struct BinOp {
    pub left: Box<RegexExp>,
    pub right: Box<RegexExp>,
    pub op: BinaryOperator,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            BinaryOperator::Concat => write!(f, "({}{})", self.left, self.right),
            BinaryOperator::Union => write!(f, "({}|{})", self.left, self.right),
        }
    }
}
