#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TokenType {
    Identifier,

    //keywords
    Let,
    If,
    Else,
    While,
    For,
    Print,
    In,
    Elif,

    New,
    Function,
    Type,
    Inherits,
    Constant,
    // Protocol,
    Extends,
    Return,

    Object,
    String,
    Boolean,
    Number,

    // literals
    BooleanLiteral,
    NumberLiteral,
    StringLiteral,

    // operators
    Lpar,
    Rpar,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    ColonAssign,
    Or,
    And,
    Not,
    Equal,
    EqualEqual,
    NotEqual,
    Plus,
    Minus,
    Times,
    Div,
    At,
    AtAt,
    Arrow,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Colon,
    Semicolon,
    Comma,

    Dot,

    __Whitespace__,
}
