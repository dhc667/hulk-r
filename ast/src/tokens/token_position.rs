#[derive(Copy, Clone, Debug, Hash)]
pub struct TokenPosition {
    pub start: usize,
    pub end: usize,
}

impl TokenPosition {
    pub fn new(start: usize, end: usize) -> Self {
        TokenPosition { start, end }
    }
}

pub trait TokenPositionTrait {
    fn position(&self) -> usize;
}

impl TokenPositionTrait for TokenPosition {
    fn position(&self) -> usize {
        self.start
    }
}
