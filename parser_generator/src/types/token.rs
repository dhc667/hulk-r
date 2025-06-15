use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Token<TokenType>
where
    TokenType: Eq + Hash,
{
    pub ty: TokenType,
    pub slice: String,
    pub start: usize,
    pub end: usize,
}

impl<TokenType> Token<TokenType>
where
    TokenType: Eq + Hash,
{
    pub fn new(ty: TokenType, slice: String, start: usize, end: usize) -> Self {
        Self {
            ty,
            slice,
            start,
            end,
        }
    }
}
