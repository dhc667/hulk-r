use std::fmt::Display;

use crate::regex_engine::regex_ast::regex_exp::RegexExp;

pub enum UnaryOperator {
    KleeneStar,
    Plus,
    Optional,
}

impl From<char> for UnaryOperator {
    fn from(c: char) -> Self {
        match c {
            '*' => UnaryOperator::KleeneStar,
            '+' => UnaryOperator::Plus,
            '?' => UnaryOperator::Optional,
            _ => panic!("Invalid unary operator character"),
        }
    }
}

pub struct UnOp {
    pub operand: Box<RegexExp>,
    pub op: UnaryOperator,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            UnaryOperator::KleeneStar => write!(f, "{}*", self.operand),
            UnaryOperator::Plus => write!(f, "{}+", self.operand),
            UnaryOperator::Optional => write!(f, "{}?", self.operand),
        }
    }
}
