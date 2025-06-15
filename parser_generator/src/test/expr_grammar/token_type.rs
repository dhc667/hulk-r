#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Plus,
    Aster,
    Identifier,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    __Whitespace__,
}
