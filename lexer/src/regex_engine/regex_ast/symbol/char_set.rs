use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharSet {
    pub chars: Vec<char>,
    pub ranges: Vec<(char, char)>,
    pub negated: bool,
}

impl CharSet {
    pub fn new(chars: Vec<char>, ranges: Vec<(char, char)>, negated: bool) -> Self {
        // Sort the characters and ranges for consistent ordering
        let mut chars = chars;
        chars.sort();

        let mut ranges = ranges;
        for range in &mut ranges {
            // Ensure ranges are in order
            if range.0 > range.1 {
                std::mem::swap(&mut range.0, &mut range.1);
            }
        }
        ranges.sort();
        CharSet {
            chars,
            ranges,
            negated,
        }
    }
}

impl PartialEq<char> for CharSet {
    fn eq(&self, other: &char) -> bool {
        let in_range = self
            .ranges
            .iter()
            .any(|&(start, end)| *other >= start && *other <= end);
        let in_set = self.chars.binary_search(other).is_ok();
        return self.negated ^ (in_range || in_set);
    }
}

impl Hash for CharSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut repr = String::new();
        if self.negated {
            repr.push('^');
        }
        for c in &self.chars {
            repr.push(*c);
        }
        for &(start, end) in &self.ranges {
            repr.push(start);
            repr.push('-');
            repr.push(end);
        }
        repr.hash(state);
    }
}
