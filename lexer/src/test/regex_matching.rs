use crate::regex_engine::regex::{DFAMatcher, NFAMatcher};

#[test]
pub fn match_literal_1() {
    let dfa_matcher = DFAMatcher::new("a");
    let nfa_matcher = NFAMatcher::new("a");

    assert!(!dfa_matcher.matches("ab"));
    assert!(dfa_matcher.matches("a"));
    assert!(!dfa_matcher.matches("b"));
    assert!(!dfa_matcher.matches("abc"));

    assert!(!nfa_matcher.matches("ab"));
    assert!(nfa_matcher.matches("a"));
    assert!(!nfa_matcher.matches("b"));
    assert!(!nfa_matcher.matches("abc"));
}

#[test]
pub fn match_literal_2() {
    let dfa_matcher = DFAMatcher::new("ab");
    let nfa_matcher = NFAMatcher::new("ab");

    assert!(dfa_matcher.matches("ab"));
    assert!(!dfa_matcher.matches("a"));
    assert!(!dfa_matcher.matches("b"));
    assert!(!dfa_matcher.matches("abc"));

    assert!(nfa_matcher.matches("ab"));
    assert!(!nfa_matcher.matches("a"));
    assert!(!nfa_matcher.matches("b"));
    assert!(!nfa_matcher.matches("abc"));
}

#[test]
pub fn match_identifier() {
    let dfa_matcher = DFAMatcher::new(r"[a-zA-Z][a-zA-Z0-9]*");
    let nfa_matcher = NFAMatcher::new(r"[a-zA-Z][a-zA-Z0-9]*");

    assert!(dfa_matcher.matches("abc"));
    assert!(dfa_matcher.matches("a1b2c3"));
    assert!(!dfa_matcher.matches("_abc123"));
    assert!(!dfa_matcher.matches("123abc"));
    assert!(!dfa_matcher.matches("abc!@#"));

    assert!(nfa_matcher.matches("abc"));
    assert!(nfa_matcher.matches("a1b2c3"));
    assert!(!nfa_matcher.matches("_abc123"));
    assert!(!nfa_matcher.matches("123abc"));
    assert!(!nfa_matcher.matches("abc!@#"));
}

#[test]
pub fn match_optional_prefix() {
    let dfa_matcher = DFAMatcher::new(r"(a|c)?");
    let nfa_matcher = NFAMatcher::new(r"(a|c)?");

    assert!(dfa_matcher.matches(""));
    assert!(dfa_matcher.matches("a"));
    assert!(dfa_matcher.matches("c"));
    assert!(!dfa_matcher.matches("abcc"));

    assert!(nfa_matcher.matches(""));
    assert!(nfa_matcher.matches("a"));
    assert!(nfa_matcher.matches("c"));
    assert!(!nfa_matcher.matches("abcc"));
}

#[test]
pub fn match_optional_suffix() {
    let dfa_matcher = DFAMatcher::new(r"a(b)?");
    let nfa_matcher = NFAMatcher::new(r"a(b)?");

    assert!(dfa_matcher.matches("a"));
    assert!(dfa_matcher.matches("ab"));
    assert!(!dfa_matcher.matches("ac"));
    assert!(!dfa_matcher.matches("abc"));
    assert!(!dfa_matcher.matches("aabc"));

    assert!(nfa_matcher.matches("a"));
    assert!(nfa_matcher.matches("ab"));
    assert!(!nfa_matcher.matches("ac"));
    assert!(!nfa_matcher.matches("abc"));
    assert!(!nfa_matcher.matches("aabc"));
}

#[test]
pub fn match_optional_suffix_2() {
    let dfa_matcher = DFAMatcher::new(r"a(bc)?");
    let nfa_matcher = NFAMatcher::new(r"a(bc)?");

    assert!(dfa_matcher.matches("a"));
    assert!(dfa_matcher.matches("abc"));
    assert!(!dfa_matcher.matches("ac"));
    assert!(!dfa_matcher.matches("aabc"));

    assert!(nfa_matcher.matches("a"));
    assert!(nfa_matcher.matches("abc"));
    assert!(!nfa_matcher.matches("ac"));
    assert!(!nfa_matcher.matches("aabc"));
}

#[test]
pub fn match_integer() {
    let regex = r"(\+|\-)?[0-9]+";
    let dfa_matcher = DFAMatcher::new(regex);
    let nfa_matcher = NFAMatcher::new(regex);

    assert!(dfa_matcher.matches("123"));
    assert!(dfa_matcher.matches("-123"));
    assert!(dfa_matcher.matches("+123"));
    assert!(!dfa_matcher.matches("123.456"));
    assert!(!dfa_matcher.matches("abc"));
    assert!(!dfa_matcher.matches("123abc"));
    assert!(!dfa_matcher.matches("123.456.789"));

    assert!(nfa_matcher.matches("123"));
    assert!(nfa_matcher.matches("-123"));
    assert!(nfa_matcher.matches("+123"));
    assert!(!nfa_matcher.matches("123.456"));
    assert!(!nfa_matcher.matches("abc"));
    assert!(!nfa_matcher.matches("123abc"));
    assert!(!nfa_matcher.matches("123.456.789"));
}

#[test]
pub fn match_float() {
    let regex = r"(\+|\-)?[0-9]+(\.[0-9]*)?";
    let dfa_matcher = DFAMatcher::new(regex);
    let nfa_matcher = NFAMatcher::new(regex);

    assert!(dfa_matcher.matches("123"));
    assert!(dfa_matcher.matches("123.456"));
    assert!(dfa_matcher.matches("123."));
    assert!(dfa_matcher.matches("-123.456"));
    assert!(dfa_matcher.matches("+123."));
    assert!(!dfa_matcher.matches(".456"));
    assert!(!dfa_matcher.matches("abc"));
    assert!(!dfa_matcher.matches("123abc"));
    assert!(!dfa_matcher.matches("123.456.789"));
    assert!(!dfa_matcher.matches("."));
    assert!(!dfa_matcher.matches(""));
    assert!(dfa_matcher.matches("123."));
    assert!(!dfa_matcher.matches(".456"));
    assert!(!dfa_matcher.matches("abc.123"));
    assert!(!dfa_matcher.matches("123.abc"));
    assert!(!dfa_matcher.matches("123.456.789"));
    assert!(!dfa_matcher.matches("123.456.789.0"));
    assert!(!dfa_matcher.matches("123.456.789.0.1"));

    assert!(nfa_matcher.matches("123"));
    assert!(nfa_matcher.matches("123.456"));
    assert!(nfa_matcher.matches("123."));
    assert!(nfa_matcher.matches("-123.456"));
    assert!(nfa_matcher.matches("+123."));
    assert!(!nfa_matcher.matches(".456"));
    assert!(!nfa_matcher.matches("abc"));
    assert!(!nfa_matcher.matches("123abc"));
    assert!(!nfa_matcher.matches("123.456.789"));
    assert!(!nfa_matcher.matches("."));
    assert!(!nfa_matcher.matches(""));
    assert!(nfa_matcher.matches("123."));
    assert!(!nfa_matcher.matches(".456"));
    assert!(!nfa_matcher.matches("abc.123"));
    assert!(!nfa_matcher.matches("123.abc"));
    assert!(!nfa_matcher.matches("123.456.789"));
    assert!(!nfa_matcher.matches("123.456.789.0"));
    assert!(!nfa_matcher.matches("123.456.789.0.1"));
}
