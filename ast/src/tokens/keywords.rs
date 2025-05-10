use std::fmt::Display;

use super::*;

#[derive(Clone, Copy)]
pub enum Keyword {
    Let(TokenPosition),
    If(TokenPosition),
    Else(TokenPosition),
    While(TokenPosition),
    For(TokenPosition),
    Print(TokenPosition),
    In(TokenPosition),
    Elif(TokenPosition),

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
            Keyword::Print(_) => write!(f, "print"),
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
        }
    }
}
