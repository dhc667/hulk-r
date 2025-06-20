use crate::regex_engine::regex_ast::symbol::{CharSet, Symbol, symbol::SymbolSet};

#[test]
pub fn charset_matching() {
    let c = 'e';
    let set = CharSet {
        ranges: vec![('d', 'f')],
        negated: false,
    };

    assert!(Symbol::Char(c) == c);
    assert!(SymbolSet::CharSet(set.clone()) == c);
    assert!(SymbolSet::Dot == c);
    assert!(Symbol::Epsilon != c);
    assert!(set.clone() == c);
}

#[test]
pub fn charset_negation() {
    let c = 'x';
    let set = CharSet {
        ranges: vec![('d', 'f')],
        negated: true,
    };

    assert!(Symbol::Char(c) == c);
    assert!(SymbolSet::CharSet(set.clone()) == c);
    assert!(SymbolSet::Dot == c);
    assert!(Symbol::Epsilon != c);
    assert!(set.clone() == c);
}
