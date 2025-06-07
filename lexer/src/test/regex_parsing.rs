use crate::RegexParser;

#[test]
fn test_simple_regex() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"a|b").unwrap();

    assert_eq!(regex.to_string(), "(a | b)");
}

#[test]
fn test_charset_negated() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[^a-zA-Z0-9]").unwrap();

    assert_eq!(regex.to_string(), "[^0-9A-Za-z]");
}

#[test]
fn test_charset_multiple_ranges() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[a-zA-Z0-9]").unwrap();

    assert_eq!(regex.to_string(), "[0-9A-Za-z]");
}

#[test]
fn test_charset_escaped_caret() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"\+[^a-z]").unwrap();

    assert_eq!(regex.to_string(), "+ [^a-z]");
}

#[test]
fn test_parse_whitespace() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"\s").unwrap();

    assert_eq!(regex.to_string(), " ");
}

#[test]
fn test_parse_tab() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"\tt").unwrap();

    assert_eq!(regex.to_string(), "\t t");
}

#[test]
fn test_parse_newline() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"\n").unwrap();

    assert_eq!(regex.to_string(), "\n");
}

#[test]
fn parse_kleene_star() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"a*").unwrap();

    assert_eq!(regex.to_string(), "a*");
}

#[test]
fn parse_identifier() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[a-zA-Z][a-zA-z0-9]+").unwrap();

    assert_eq!(regex.to_string(), "[A-Za-z] [0-9A-za-z]+");
}
