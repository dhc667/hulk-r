use std::{fmt::Display, usize};

use super::*;

#[derive(Debug)]
pub struct NumberLiteral {
    pub position: TokenPosition,
    pub value: f64,
}

impl NumberLiteral {
    pub fn new(start: usize, end: usize, value: &str) -> Self {
        NumberLiteral {
            position: TokenPosition::new(start, end),
            value: value.parse::<f64>().unwrap(),
        }
    }
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub enum BooleanLiteral {
    True(TokenPosition),
    False(TokenPosition),
}

impl BooleanLiteral {
    pub fn new(start: usize, end: usize, value: bool) -> BooleanLiteral {
        let position = TokenPosition::new(start, end);

        match value {
            true => BooleanLiteral::True(position),
            false => BooleanLiteral::False(position),
        }
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Self::True(_) => "true",
                Self::False(_) => "false",
            }
        })
    }
}

#[derive(Debug)]
pub struct StringLiteral {
    pub position: TokenPosition,
    pub string: String,
}

impl StringLiteral {
    pub fn new(start: usize, end: usize, string: &str) -> Self {
        Self {
            position: TokenPosition::new(start, end),
            string: string
                .strip_prefix("\"")
                .expect("String literals must start with '\"' character: parser problem")
                .strip_suffix("\"")
                .expect("String literals must end with '\"' character: parser problem")
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\t", "\t"),
        }
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.string)
    }
}
