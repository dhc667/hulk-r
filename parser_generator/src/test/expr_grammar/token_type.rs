#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Plus,
    Aster,
    Identifier,
    LParen,
    RParen,
    Comma,
    __Whitespace__,
} 