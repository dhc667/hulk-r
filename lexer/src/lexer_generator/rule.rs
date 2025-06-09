use std::hash::Hash;

pub struct Rule<TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
{
    pub token_kind: TokenKind,
    pub pattern: String,
    pub skip: bool,
}

impl<TokenKind> Rule<TokenKind>
where
    TokenKind: Clone + PartialEq + Hash + Eq,
{
    pub fn new(token_kind: TokenKind, pattern: String) -> Self {
        Rule {
            token_kind,
            pattern,
            skip: false,
        }
    }

    pub fn new_skip(token_kind: TokenKind, pattern: String) -> Self {
        Rule {
            token_kind,
            pattern,
            skip: true,
        }
    }
}
