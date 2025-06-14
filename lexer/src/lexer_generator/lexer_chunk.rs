use std::hash::Hash;

/// # Description
/// This module defines a `LexerChunk` struct that represents a chunk of text recognized by the lexer.
/// ## Fields:
/// - `ty`: The type of token this chunk represents.
/// - `slice`: The actual text slice that this chunk represents.
/// - `line`: The line number in the source text where this chunk starts.
/// - `start`: The starting index of this chunk in the source text.
/// - `end`: The ending index of this chunk in the source text.
/// ## Methods:
/// - `new`: Creates a new `LexerChunk` with the specified type, slice, line, start, and end indices.
/// - `is_empty`: Checks if the chunk is empty (i.e., start and end indices are the same).
#[derive(Debug)]
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
    /// Creates a new `LexerChunk` with the specified type, slice, line, start, and end indices.
    /// # Arguments
    /// * `ty`: The type of token this chunk represents.
    /// * `slice`: The actual text slice that this chunk represents.
    /// * `line`: The line number in the source text where this chunk starts.
    /// * `start`: The starting index of this chunk in the source text.
    /// * `end`: The ending index of this chunk in the source text.
    /// # Returns
    /// A new `LexerChunk` instance.
    pub fn new(ty: TokenKind, slice: &'a str, line: usize, start: usize, end: usize) -> Self {
        LexerChunk {
            ty,
            slice,
            line,
            start,
            end,
        }
    }

    /// Checks if the chunk is empty (i.e., start and end indices are the same).
    /// # Returns
    /// `true` if the chunk is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}
