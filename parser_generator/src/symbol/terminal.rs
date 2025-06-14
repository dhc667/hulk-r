use std::hash::Hash;

use crate::Token;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct TerminalId(pub usize);

impl TerminalId {
    pub fn id_string(&self) -> String {
        self.0.to_string()
    }

}

pub struct Terminal<TokenType: Eq + Hash, R> {
    pub id: TerminalId,
    pub token_type: TokenType,
    pub compute: Box<dyn for<'a> Fn(&'a Token<TokenType>) -> R>,
}

impl<TokenType: Eq + Hash + std::fmt::Debug, R: std::fmt::Debug> std::fmt::Debug for Terminal<TokenType, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Terminal").field("id", &self.id).field("token_type", &self.token_type).finish()
    }
}

impl<TokenType: Eq + Hash, R> Terminal<TokenType, R> {
    pub fn new(id: usize, token_type: TokenType, compute: impl Fn(&Token<TokenType>) -> R + 'static) -> Self {
        return Self {
            id: TerminalId(id),
            token_type,
            compute: Box::new(compute),
        };
    }
}

impl<TokenType: Eq + Hash, R> PartialEq for Terminal<TokenType, R> {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

impl<TokenType: Eq + Hash, R> Hash for Terminal<TokenType, R> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.token_type.hash(state);
    }
}
