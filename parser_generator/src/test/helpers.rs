use std::{fmt::Debug, hash::Hash};

use lexer::lexer_generator::{lexer::Lexer, lexer_chunk::LexerChunk};

use crate::{ParseError, Parser, Token};

pub fn parse<T: Hash + Eq + Copy + Debug, R>(
    lexer_parser: impl FnOnce() -> (Lexer<T>, Parser<T, R>),
    input: &str,
) -> Result<R, ParseError<T>> {
    let (lexer, parser) = lexer_parser();

    let tokens = lexer
        .split(input)
        .unwrap()
        .iter()
        .map(chunk_to_token)
        .collect();

    let answ = parser.parse(tokens);

    answ
}
pub fn chunk_to_token<T: Eq + Hash + Copy>(chunk: &LexerChunk<T>) -> Token<T> {
    Token::new(chunk.ty, chunk.slice.to_string(), chunk.start, chunk.end)
}
