use super::*;

pub enum BinaryOperator {
    Plus(TokenPosition),
    Minus(TokenPosition),
    Divide(TokenPosition),
    FloorDivide(TokenPosition),
    Times(TokenPosition),
    Modulo(TokenPosition),

    Equal(TokenPosition),
    ColonEqual(TokenPosition),
}

pub enum UnaryOperator {
    Plus(TokenPosition),
    Minus(TokenPosition),
}

pub enum GroupingOperator {
    OpenParen(TokenPosition),
    CloseParen(TokenPosition),
    OpenBrace(TokenPosition),
    CloseBrace(TokenPosition),
}
