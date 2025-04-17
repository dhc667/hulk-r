use super::*;

pub enum Keyword {
    Let(TokenPosition),
    If(TokenPosition),
    Else(TokenPosition),
    While(TokenPosition),
    Print(TokenPosition),
    In(TokenPosition),
    Elif(TokenPosition),
}
