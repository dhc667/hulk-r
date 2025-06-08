use crate::lexer_generator::rule::Rule;

pub struct Lexer<TokenKind, TokenType>
where
    TokenKind: Clone + PartialEq,
{
    pub rules: Vec<Rule<TokenKind, TokenType>>,
}

impl<TokenKind, TokenType> Lexer<TokenKind, TokenType>
where
    TokenKind: Clone + PartialEq,
{
    pub fn new(rules: Vec<Rule<TokenKind, TokenType>>) -> Self {
        Lexer { rules }
    }
}
