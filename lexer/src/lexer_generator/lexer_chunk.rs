use std::hash::Hash;

pub struct LexerChunk<'a, TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
{
    pub ty: TokenKind,
    pub slice: &'a str,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl<'a, TokenKind> LexerChunk<'a, TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
{
    pub fn new(ty: TokenKind, slice: &'a str, line: usize, start: usize, end: usize) -> Self {
        LexerChunk {
            ty,
            slice,
            line,
            start,
            end,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}
