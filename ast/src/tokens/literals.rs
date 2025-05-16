use std::fmt::Display;

use super::*;

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
