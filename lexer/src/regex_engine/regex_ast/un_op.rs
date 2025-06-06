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
