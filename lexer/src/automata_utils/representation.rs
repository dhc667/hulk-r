use std::collections::HashSet;

/// # Description
/// This module provides utility functions to convert between a `HashSet<usize>` and a `String` representation.
/// # Functions
/// - `to_str`: Converts a `HashSet<usize>` to a `String` representation.
/// - `to_set`: Converts a `String` representation back to a `HashSet<usize>`.

/// # Description
/// Converts a `HashSet<usize>` to a `String` representation. It sorts the elements and joins them with a space.
/// It is useful for creating a human-readable representation of the set.
/// It is deterministic, meaning the same set will always produce the same string.
pub fn to_str(s: &HashSet<usize>) -> String {
    let mut v: Vec<usize> = s.iter().cloned().collect();
    v.sort_unstable();
    let repr: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    repr.join(" ")
}

/// # Description
/// Converts a `String` representation back to a `HashSet<usize>`. It splits the string by spaces,
/// parses each part as a `usize`, and collects them into a `HashSet`.
/// It is useful for parsing a human-readable representation back into a set.
/// It is intended to be used with strings that are produced by `to_str`.
pub fn to_set(s: &String) -> HashSet<usize> {
    let nums: HashSet<usize> = s.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
    nums
}
