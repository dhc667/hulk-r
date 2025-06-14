#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum TokenType {
    Number,
    Plus,
    Times,
    Lpar,
    Rpar,
    Comma,

    __Whitespace__,
}
