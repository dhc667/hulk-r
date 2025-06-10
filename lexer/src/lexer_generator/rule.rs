use std::hash::Hash;

/// # Description
/// This module defines a `Rule` struct that represents a rule in a lexer.
/// ## Fields:
/// - `token_kind`: The kind of token this rule matches.
/// - `pattern`: The regex pattern that defines the rule.
/// - `skip`: A boolean indicating whether this rule should be skipped during tokenization.
/// ## Methods:
/// - `new`: Creates a new `Rule` with the specified token kind and pattern.
/// - `new_skip`: Creates a new `Rule` that should be skipped during tokenization.
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
    /// Creates a new `Rule` with the specified token kind and pattern.
    /// # Arguments
    /// * `token_kind`: The kind of token this rule matches.
    /// * `pattern`: The regex pattern that defines the rule.
    /// # Returns
    /// A new `Rule` instance.
    pub fn new(token_kind: TokenKind, pattern: String) -> Self {
        Rule {
            token_kind,
            pattern,
            skip: false,
        }
    }

    /// Creates a new `Rule` that should be skipped during tokenization.
    /// # Arguments
    /// * `token_kind`: The kind of token this rule matches.
    /// * `pattern`: The regex pattern that defines the rule.
    /// # Returns
    /// A new `Rule` instance that is marked to be skipped.
    pub fn new_skip(token_kind: TokenKind, pattern: String) -> Self {
        Rule {
            token_kind,
            pattern,
            skip: true,
        }
    }
}
