use std::fmt::Display;

use crate::token_position::TokenPositionTrait;

use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Keyword {
    Let(TokenPosition),
    If(TokenPosition),
    Else(TokenPosition),
    While(TokenPosition),
    For(TokenPosition),
    In(TokenPosition),
    Elif(TokenPosition),

    New(TokenPosition),
    Function(TokenPosition),
    Type(TokenPosition),
    Inherits(TokenPosition),
    Constant(TokenPosition),
    Protocol(TokenPosition),
    Extends(TokenPosition),
    Return(TokenPosition),
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Let(_) => write!(f, "let"),
            Keyword::If(_) => write!(f, "if"),
            Keyword::Else(_) => write!(f, "else"),
            Keyword::While(_) => write!(f, "while"),
            Keyword::In(_) => write!(f, "in"),
            Keyword::Elif(_) => write!(f, "elif"),
            Keyword::Function(_) => write!(f, "function"),
            Keyword::Type(_) => write!(f, "type"),
            Keyword::Constant(_) => write!(f, "constant"),
            Keyword::Protocol(_) => write!(f, "protocol"),
            Keyword::Inherits(_) => write!(f, "inherits"),
            Keyword::Extends(_) => write!(f, "extends"),
            Keyword::Return(_) => write!(f, "return"),
            Keyword::For(_) => write!(f, "for"),
            Keyword::New(_) => write!(f, "new"),
        }
    }
}

impl TokenPositionTrait for Keyword {
    fn position(&self) -> usize {
        match self {
            Keyword::Let(pos)
            | Keyword::If(pos)
            | Keyword::Else(pos)
            | Keyword::While(pos)
            | Keyword::In(pos)
            | Keyword::Elif(pos)
            | Keyword::Function(pos)
            | Keyword::Type(pos)
            | Keyword::Constant(pos)
            | Keyword::Protocol(pos)
            | Keyword::Inherits(pos)
            | Keyword::Extends(pos)
            | Keyword::Return(pos)
            | Keyword::For(pos)
            | Keyword::New(pos) => pos.position(),
        }
    }
}
