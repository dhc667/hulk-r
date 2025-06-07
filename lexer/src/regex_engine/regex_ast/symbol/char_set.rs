use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharSet {
    pub ranges: Vec<(char, char)>,
    pub negated: bool,
}

impl CharSet {
    pub fn new(ranges: Vec<(char, char)>, negated: bool) -> Self {
        let mut ranges = ranges;
        for range in &mut ranges {
            // Ensure ranges are in order
            if range.0 > range.1 {
                std::mem::swap(&mut range.0, &mut range.1);
            }
        }
        ranges.sort();
        CharSet { ranges, negated }
    }
}

impl PartialEq<char> for CharSet {
    fn eq(&self, other: &char) -> bool {
        self.negated
            ^ self
                .ranges
                .iter()
                .any(|&(start, end)| *other >= start && *other <= end)
    }
}

impl Display for CharSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::new();
        repr.push('[');
        if self.negated {
            repr.push('^');
        }
        for &(start, end) in &self.ranges {
            repr.push(start);
            repr.push('-');
            repr.push(end);
        }
        repr.push(']');
        write!(f, "{}", repr)
    }
}

impl Hash for CharSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let repr = self.to_string();
        repr.hash(state);
    }
}
