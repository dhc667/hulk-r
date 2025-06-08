use std::hash::Hash;

pub struct Rule<TokenKind, TokenType>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
{
    pub token_kind: TokenKind,
    pub pattern: String,
    pub action: Option<Box<dyn Fn(&TokenKind, &str, usize, usize, usize) -> TokenType>>,
}

impl<TokenKind, TokenType> Rule<TokenKind, TokenType>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
{
    pub fn new<F>(token_kind: TokenKind, pattern: String, action: F) -> Self
    where
        F: Fn(&TokenKind, &str, usize, usize, usize) -> TokenType + 'static,
    {
        Rule {
            token_kind,
            pattern,
            action: Some(Box::new(action)),
        }
    }

    pub fn new_skip<F>(token_kind: TokenKind, pattern: String) -> Self
    where
        F: Fn(TokenKind, &str, usize, usize) -> TokenType + 'static,
    {
        Rule {
            token_kind,
            pattern,
            action: None,
        }
    }
}
