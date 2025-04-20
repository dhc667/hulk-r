use std::fmt::Display;

use super::*;

#[derive(Clone, Copy)]
pub enum Keyword {
    Let(TokenPosition),
    If(TokenPosition),
    Else(TokenPosition),
    While(TokenPosition),
    Print(TokenPosition),
    In(TokenPosition),
    Elif(TokenPosition),
    True(TokenPosition),
    False(TokenPosition),
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
            Keyword::True(_) => write!(f, "true"),
            Keyword::False(_) => write!(f, "false"),
        }
    }
}
