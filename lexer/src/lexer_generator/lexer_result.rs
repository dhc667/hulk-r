use error_handler::error::error::HulkError;

use crate::lexer_generator::lexer_chunk::LexerChunk;

use std::{fmt::Debug, hash::Hash};

/// # Description
/// This module defines a `LexerResult` struct that encapsulates the results of a lexical analysis operation.
/// It contains a vector of tokens and a vector of error messages encountered during the analysis.
/// ## Fields:
/// - `tokens`: A vector of `LexerChunk` instances representing the tokens found in the input string.
/// - `errors`: A vector of error messages encountered during the tokenization process.
/// ## Methods:
/// - `new`: Constructs a new `LexerResult` instance with the provided tokens and errors.
/// - `new`: Constructs a new `LexerResult` instance with the provided tokens and errors.
pub struct LexerResult<'a, TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq + Debug,
{
    pub tokens: Vec<LexerChunk<'a, TokenKind>>,
    pub errors: Vec<HulkError>,
}

impl<'a, TokenKind> LexerResult<'a, TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq + Debug,
{
    /// Constructs a new `LexerResult` instance with the provided tokens and errors.
    /// # Arguments
    /// - `tokens`: A vector of `LexerChunk` instances representing the tokens found in the input string.
    /// - `errors`: A vector of error messages encountered during the tokenization process.
    /// # Returns
    /// A new `LexerResult` instance containing the provided tokens and errors.
    pub fn new(tokens: Vec<LexerChunk<'a, TokenKind>>, errors: Vec<HulkError>) -> Self {
        Self { tokens, errors }
    }
}
