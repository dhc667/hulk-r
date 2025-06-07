use crate::RegexParser;

#[test]
fn test_simple_regex() {
    let parser = RegexParser::new();
    parser.parse(r"a|b").unwrap();
}

#[test]
fn test_charset_negated() {
    let parser = RegexParser::new();
    parser.parse(r"[^a-zA-Z0-9]").unwrap();
}

#[test]
fn test_charset_multiple_ranges() {
    let parser = RegexParser::new();
    parser.parse(r"[a-zA-Z0-9]").unwrap();
}

#[test]
fn test_charset_escaped_caret() {
    let parser = RegexParser::new();
    parser.parse(r"\+[^a-z]").unwrap();
}

#[test]
fn test_parse_whitespace() {
    let parser = RegexParser::new();
    parser.parse(r"\s").unwrap();
}

#[test]
fn test_parse_tab() {
    let parser = RegexParser::new();
    parser.parse(r"t").unwrap();
}
