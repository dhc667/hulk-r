use std::{fmt::Debug, hash::Hash};

use error_handler::error::error::HulkError;
use lexer::lexer_generator::{lexer::Lexer, lexer_chunk::LexerChunk, rule::Rule};

use crate::{
    ParseError, Parser, Token,
    grammar::{DefineLexer, Lex},
};

pub fn parse<T: Hash + Eq + Copy + Debug, R>(
    lexer_parser: impl FnOnce() -> (LexerWrapper<T>, Parser<T, R>),
    input: &str,
) -> Result<R, ParseError<T>> {
    let (lexer, parser) = lexer_parser();

    let tokens = lexer.split(input).unwrap();

    let answ = parser.parse(tokens);

    answ
}

pub fn chunk_to_token<T: Eq + Hash + Copy>(chunk: &LexerChunk<T>) -> Token<T> {
    Token::new(chunk.ty, chunk.slice.to_string(), chunk.start, chunk.end)
}

pub struct LexerWrapper<T: Hash + Eq + Copy + Debug> {
    lexer: Lexer<T>,
}

impl<T: Hash + Eq + Copy + Debug> LexerWrapper<T> {
    pub fn new(lexer: Lexer<T>) -> Self {
        Self { lexer }
    }
}

impl<T: Hash + Eq + Copy + Debug> Lex<T> for LexerWrapper<T> {
    fn split(&self, input: &str) -> Result<Vec<Token<T>>, Vec<HulkError>> {
        let answ = self.lexer.split(input);
        let answ = answ.map(|chks| chks.iter().map(chunk_to_token).collect());
        let answ = answ.map_err(|(_, errs)| errs);

        answ
    }
}

pub struct LexerDefiner<T: Hash + Eq + Copy + Debug> {
    rules: Vec<Rule<T>>,
}

impl<T: Hash + Eq + Copy + Debug> DefineLexer<T, LexerWrapper<T>> for LexerDefiner<T> {
    fn new() -> Self {
        Self { rules: Vec::new() }
    }

    fn rule(&mut self, tok_ty: T, pattern: String) {
        self.rules.push(Rule::new(tok_ty, pattern));
    }

    fn skip_rule(&mut self, tok_ty: T, pattern: String) {
        self.rules.push(Rule::new_skip(tok_ty, pattern));
    }

    fn compile(self) -> LexerWrapper<T> {
        LexerWrapper::new(Lexer::new(self.rules))
    }
}
