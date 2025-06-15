use std::fmt::Display;
use std::fmt::Formatter;

use crate::tokens::token_position::TokenPositionTrait;

use super::*;

#[derive(Debug)]
pub enum BinaryOperator {
    Plus(TokenPosition),
    Minus(TokenPosition),
    Divide(TokenPosition),
    FloorDivide(TokenPosition),
    Times(TokenPosition),
    Modulo(TokenPosition),

    At(TokenPosition),
    AtAt(TokenPosition),

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

impl TokenPositionTrait for BinaryOperator {
    fn position(&self) -> usize {
        match self {
            BinaryOperator::Plus(pos) => pos.start,
            BinaryOperator::Minus(pos) => pos.start,
            BinaryOperator::Divide(pos) => pos.start,
            BinaryOperator::FloorDivide(pos) => pos.start,
            BinaryOperator::Times(pos) => pos.start,
            BinaryOperator::Modulo(pos) => pos.start,
            BinaryOperator::At(pos) => pos.start,
            BinaryOperator::AtAt(pos) => pos.start,
            BinaryOperator::EqualEqual(pos) => pos.start,
            BinaryOperator::Less(pos) => pos.start,
            BinaryOperator::LessEqual(pos) => pos.start,
            BinaryOperator::Greater(pos) => pos.start,
            BinaryOperator::GreaterEqual(pos) => pos.start,
            BinaryOperator::NotEqual(pos) => pos.start,
            BinaryOperator::Or(pos) => pos.start,
            BinaryOperator::And(pos) => pos.start,
            BinaryOperator::Equal(pos) => pos.start,
            BinaryOperator::ColonEqual(pos) => pos.start,
        }
    }
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
            BinaryOperator::At(_) => write!(f, "@"),
            BinaryOperator::AtAt(_) => write!(f, "@@"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Plus(TokenPosition),
    Minus(TokenPosition),
    Not(TokenPosition),
}

impl TokenPositionTrait for UnaryOperator {
    fn position(&self) -> usize {
        match self {
            UnaryOperator::Plus(pos) => pos.start,
            UnaryOperator::Minus(pos) => pos.start,
            UnaryOperator::Not(pos) => pos.start,
        }
    }
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

#[derive(Debug)]
pub enum GroupingOperator {
    OpenParen(TokenPosition),
    CloseParen(TokenPosition),
    OpenBrace(TokenPosition),
    CloseBrace(TokenPosition),
    OpenBracket(TokenPosition),
    CloseBracket(TokenPosition),
}

impl TokenPositionTrait for GroupingOperator {
    fn position(&self) -> usize {
        match self {
            GroupingOperator::OpenParen(pos) => pos.start,
            GroupingOperator::CloseParen(pos) => pos.start,
            GroupingOperator::OpenBrace(pos) => pos.start,
            GroupingOperator::CloseBrace(pos) => pos.start,
            GroupingOperator::OpenBracket(pos) => pos.start,
            GroupingOperator::CloseBracket(pos) => pos.start,
        }
    }
}

impl Display for GroupingOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupingOperator::OpenParen(_) => write!(f, "("),
            GroupingOperator::CloseParen(_) => write!(f, ")"),
            GroupingOperator::OpenBrace(_) => write!(f, "{{"),
            GroupingOperator::CloseBrace(_) => write!(f, "}}"),
            GroupingOperator::OpenBracket(_) => write!(f, "["),
            GroupingOperator::CloseBracket(_) => write!(f, "]"),
        }
    }
}

#[derive(Debug)]
pub struct ArrowOperator {
    pub position: TokenPosition,
}

impl ArrowOperator {
    pub fn new(position: TokenPosition) -> Self {
        Self { position }
    }
}

impl TokenPositionTrait for ArrowOperator {
    fn position(&self) -> usize {
        self.position.start
    }
}

#[derive(Debug)]
pub struct DotOperator {
    pub position: TokenPosition,
}

impl DotOperator {
    pub fn new(position: TokenPosition) -> Self {
        Self { position }
    }
}

impl Display for DotOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ".")
    }
}

impl TokenPositionTrait for DotOperator {
    fn position(&self) -> usize {
        self.position.start
    }
}
