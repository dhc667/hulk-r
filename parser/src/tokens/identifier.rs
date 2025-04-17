use super::*;

pub struct Identifier {
    pub position: TokenPosition,
    pub id: String,
}

impl Identifier {
    pub fn new(start: usize, end: usize, id: &str) -> Self {
        Identifier {
            position: TokenPosition::new(start, end),
            id: id.to_string(),
        }
    }
}
