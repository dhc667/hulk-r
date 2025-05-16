#[derive(Copy, Clone)]
pub struct TokenPosition {
    pub start: usize,
    pub end: usize,
}

impl TokenPosition {
    pub fn new(start: usize, end: usize) -> Self {
        TokenPosition { start, end }
    }
}
