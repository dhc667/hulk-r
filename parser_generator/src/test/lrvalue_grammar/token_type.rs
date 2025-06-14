#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum TokenType {
    Identifier,
    Aster,
    Equal,

    __Whitespace__,
}
