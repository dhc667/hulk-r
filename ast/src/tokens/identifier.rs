use std::fmt::Display;

use super::*;

pub struct Identifier {
    pub position: TokenPosition,
    pub id: String,
    pub context_id: Option<usize>, // Optional context ID for scoping
}

impl Identifier {
    pub fn new(start: usize, end: usize, id: &str) -> Self {
        Identifier {
            position: TokenPosition::new(start, end),
            id: id.to_string(),
            context_id: None, // Default to None, can be set later
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
