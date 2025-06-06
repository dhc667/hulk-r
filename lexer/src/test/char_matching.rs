use crate::regex_engine::regex_ast::symbol::{CharSet, Symbol};

#[test]
pub fn charset_matching() {
    let c = 'a';
    let set = CharSet {
        chars: vec!['a', 'b', 'c'].into_iter().collect(),
        ranges: vec![('d', 'f')],
        negated: false,
    };

    assert!(Symbol::Char(c) == c);
    assert!(Symbol::CharSet(set.clone()) == c);
    assert!(Symbol::Dot == c);
    assert!(Symbol::Epsilon != c);
    assert!(set.clone() == c);
}

#[test]
pub fn charset_negation() {
    let c = 'x';
    let set = CharSet {
        chars: vec!['a', 'b', 'c'].into_iter().collect(),
        ranges: vec![('d', 'f')],
        negated: true,
    };

    assert!(Symbol::Char(c) == c);
    assert!(Symbol::CharSet(set.clone()) == c);
    assert!(Symbol::Dot == c);
    assert!(Symbol::Epsilon != c);
    assert!(set.clone() == c);
}
