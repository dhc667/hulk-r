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
