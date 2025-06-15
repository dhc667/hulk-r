use std::hash::Hash;

use crate::{StateId, Token};

#[derive(Debug)]
pub struct Parse<TokenType: Eq + Hash, R> {
    pub tokens: Vec<Token<TokenType>>,
    pub state_stack: Vec<StateId>,
    pub value_stack: Vec<R>,
    pub token_index: usize,
}

impl<TokenType: Eq + Hash, R> Parse<TokenType, R> {
    pub fn new(tokens: Vec<Token<TokenType>>) -> Self {
        Self {
            tokens,
            state_stack: vec![StateId(0)],
            value_stack: Vec::new(),
            token_index: 0,
        }
    }
}
