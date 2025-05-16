use std::fmt::Display;
use std::fmt::Formatter;

use super::*;

pub enum BinaryOperator {
    Plus(TokenPosition),
    Minus(TokenPosition),
    Divide(TokenPosition),
    FloorDivide(TokenPosition),
    Times(TokenPosition),
    Modulo(TokenPosition),

    EqualEqual(TokenPosition),
    Less(TokenPosition),
    LessEqual(TokenPosition),
    Greater(TokenPosition),
    GreaterEqual(TokenPosition),
    NotEqual(TokenPosition),

    Or(TokenPosition),
    And(TokenPosition),

    Equal(TokenPosition),
    ColonEqual(TokenPosition),
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Plus(_) => write!(f, "+"),
            BinaryOperator::Minus(_) => write!(f, "-"),
            BinaryOperator::Divide(_) => write!(f, "/"),
            BinaryOperator::FloorDivide(_) => write!(f, "//"),
            BinaryOperator::Times(_) => write!(f, "*"),
            BinaryOperator::Modulo(_) => write!(f, "%"),

            BinaryOperator::Equal(_) => write!(f, "="),
            BinaryOperator::ColonEqual(_) => write!(f, ":="),

            BinaryOperator::Less(_) => write!(f, "<"),
            BinaryOperator::LessEqual(_) => write!(f, "<="),
            BinaryOperator::Greater(_) => write!(f, ">"),
            BinaryOperator::GreaterEqual(_) => write!(f, ">="),
            BinaryOperator::EqualEqual(_) => write!(f, "=="),
            BinaryOperator::NotEqual(_) => write!(f, "!="),
            BinaryOperator::Or(_) => write!(f, "||"),
            BinaryOperator::And(_) => write!(f, "&&"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum UnaryOperator {
    Plus(TokenPosition),
    Minus(TokenPosition),
    Not(TokenPosition),
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Plus(_) => write!(f, "+"),
            UnaryOperator::Minus(_) => write!(f, "-"),
            UnaryOperator::Not(_) => write!(f, "!"),
        }
    }
}

pub enum GroupingOperator {
    OpenParen(TokenPosition),
    CloseParen(TokenPosition),
    OpenBrace(TokenPosition),
    CloseBrace(TokenPosition),
}

impl Display for GroupingOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupingOperator::OpenParen(_) => write!(f, "("),
            GroupingOperator::CloseParen(_) => write!(f, ")"),
            GroupingOperator::OpenBrace(_) => write!(f, "{{"),
            GroupingOperator::CloseBrace(_) => write!(f, "}}"),
        }
    }
}
