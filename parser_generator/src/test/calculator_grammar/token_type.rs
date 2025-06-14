#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Times,
    Div,
    Lpar,
    Rpar,

    __Whitespace__,
}
