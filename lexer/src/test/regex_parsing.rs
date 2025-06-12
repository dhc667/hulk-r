use crate::RegexParser;

#[test]
fn test_simple_regex() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"a|b").unwrap();

    assert_eq!(regex.to_string(), "(a|b)");
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

    assert_eq!(regex.to_string(), "(+[^a-z])");
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

    assert_eq!(regex.to_string(), "(\tt)");
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

    assert_eq!(regex.to_string(), "([A-Za-z][0-9A-za-z]+)");
}

#[test]
pub fn parse_literal_1() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"a").unwrap();

    assert_eq!(regex.to_string(), "a");
}

#[test]
pub fn parse_literal_2() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"ab").unwrap();

    assert_eq!(regex.to_string(), "(ab)");
}

#[test]
pub fn parse_literal_3() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"acb").unwrap();

    assert_eq!(regex.to_string(), "((ac)b)");
}

#[test]
pub fn parse_literal_4() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"a(bc)").unwrap();

    assert_eq!(regex.to_string(), "(a(bc))");
}

#[test]
pub fn parse_kleene_star_1() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"a*").unwrap();

    assert_eq!(regex.to_string(), "a*");
}

#[test]
pub fn parse_kleene_star_2() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"ab*").unwrap();

    assert_eq!(regex.to_string(), "(ab*)");
}

#[test]
pub fn parse_kleene_star_3() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"a*b").unwrap();

    assert_eq!(regex.to_string(), "(a*b)");
}

#[test]
pub fn parse_kleene_star_4() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"(ab)*").unwrap();

    assert_eq!(regex.to_string(), "(ab)*");
}

#[test]
pub fn parse_kleene_star_5() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"(a|b)*").unwrap();

    assert_eq!(regex.to_string(), "(a|b)*");
}

#[test]
pub fn parse_alpha() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[a-zA-Z]*").unwrap();

    assert_eq!(regex.to_string(), "[A-Za-z]*");
}

#[test]
pub fn parse_digit() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[0-9]*").unwrap();

    assert_eq!(regex.to_string(), "[0-9]*");
}

#[test]
pub fn parse_char_set_1() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[a-c]*").unwrap();

    assert_eq!(regex.to_string(), "[a-c]*");
}

#[test]
pub fn parse_non_alpha() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[^a-zA-Z]*").unwrap();

    assert_eq!(regex.to_string(), "[^A-Za-z]*");
}

#[test]
pub fn parse_union_1() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"a|b").unwrap();

    assert_eq!(regex.to_string(), "(a|b)");
}

#[test]
pub fn parse_union_2() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"a(b|c)").unwrap();

    assert_eq!(regex.to_string(), "(a(b|c))");
}

#[test]
pub fn parse_union_3() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"(a|b)c").unwrap();

    assert_eq!(regex.to_string(), "((a|b)c)");
}

#[test]
pub fn parse_dot() {
    let parser = RegexParser::new();
    let regex = parser.parse(r".").unwrap();

    assert_eq!(regex.to_string(), ".");
}

#[test]
pub fn parse_dot_kleene_star() {
    let parser = RegexParser::new();
    let regex = parser.parse(r".*").unwrap();

    assert_eq!(regex.to_string(), ".*");
}

#[test]
pub fn parse_plus_1() {
    let parser = RegexParser::new();
    let regex = parser.parse(r".+").unwrap();

    assert_eq!(regex.to_string(), ".+");
}

#[test]
pub fn parse_plus_2() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[0-9]+").unwrap();

    assert_eq!(regex.to_string(), "[0-9]+");
}

#[test]
pub fn parse_optional_1() {
    let parser = RegexParser::new();
    let regex = parser.parse(r".?").unwrap();

    assert_eq!(regex.to_string(), ".?");
}

#[test]
pub fn parse_optional_2() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[0-9]?").unwrap();

    assert_eq!(regex.to_string(), "[0-9]?");
}

#[test]
pub fn parse_float() {
    let parser = RegexParser::new();
    let regex = parser.parse(r"[0-9]+\.[0-9]+").unwrap();

    assert_eq!(regex.to_string(), "(([0-9]+.)[0-9]+)");
}
