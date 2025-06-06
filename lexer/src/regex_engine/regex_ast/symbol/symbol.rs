use crate::regex_engine::regex_ast::symbol::CharSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Char(char),
    CharSet(CharSet),
    Dot,
    Epsilon,
}

impl PartialEq<char> for Symbol {
    fn eq(&self, other: &char) -> bool {
        match self {
            Symbol::Char(c) => c == other,
            Symbol::CharSet(set) => set == other,
            Symbol::Dot => true,      // Dot matches any character
            Symbol::Epsilon => false, // Epsilon does not match any character
        }
    }
}
