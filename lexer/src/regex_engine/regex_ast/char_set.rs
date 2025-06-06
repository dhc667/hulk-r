use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CharSet {
    pub chars: HashSet<char>,
    pub ranges: Vec<(char, char)>,
    pub negated: bool,
}

impl PartialEq<char> for CharSet {
    fn eq(&self, other: &char) -> bool {
        let in_range = self
            .ranges
            .iter()
            .any(|&(start, end)| *other >= start && *other <= end);
        let in_set = self.chars.contains(other);
        return self.negated ^ (in_range || in_set);
    }
}
