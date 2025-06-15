use error_handler::error::error::HulkError;
use std::{fmt::Debug, hash::Hash};

use crate::Token;

pub trait Lex<T: Hash + Debug + Eq + Copy> {
    fn split(&self, input: &str) -> Result<Vec<Token<T>>, Vec<HulkError>>;
}

pub trait DefineLexer<T: Hash + Debug + Eq + Copy, Lexer: Lex<T>> {
    fn new() -> Self;

    fn rule(&mut self, tok_ty: T, pattern: String);
    fn rules(&mut self, rules: Vec<(T, String)>) {
        for (tok_ty, pattern) in rules {
            self.rule(tok_ty, pattern);
        }
    }

    fn skip_rule(&mut self, tok_ty: T, pattern: String);
    fn skip_rules(&mut self, skips: Vec<(T, String)>) {
        for (tok_ty, pattern) in skips {
            self.skip_rule(tok_ty, pattern);
        }
    }

    fn compile(self) -> Lexer;
}
