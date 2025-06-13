use std::fmt::Display;

use crate::regex_engine::regex_ast::symbol::CharSet;

/// # Description
/// An enum representing a symbol in a regular expression.
/// It can be a character or an epsilon (empty string).
/// ## Variants
/// - `Char(char)`: Represents a single character.
/// - `Epsilon`: Represents the empty string (epsilon).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Symbol {
    Char(char),
    Epsilon,
}

impl Symbol {
    pub fn as_char(&self) -> Option<&char> {
        if let Self::Char(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        Symbol::Char(c)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Char(c) => write!(f, "{}", c),
            Symbol::Epsilon => write!(f, "\\epsilon"),
        }
    }
}

/// # Description
/// An enum representing a set of symbols in a regular expression.
/// It can be a character set or a dot (which matches any character).
/// ## Variants
/// - `CharSet(CharSet)`: Represents a set of characters.
/// - `Dot`: Represents a dot, which matches any character.
pub enum SymbolSet {
    CharSet(CharSet),
    Dot,
}

impl SymbolSet {
    pub fn as_char_set(&self) -> Option<&CharSet> {
        if let Self::CharSet(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Display for SymbolSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolSet::CharSet(char_set) => {
                write!(f, "{}", char_set)
            }
            SymbolSet::Dot => write!(f, "."),
        }
    }
}

/// # Description
/// An enum representing a matchable symbol in a regular expression.
/// It can be a single symbol or a set of symbols.
/// ## Variants
/// - `Symbol(Symbol)`: Represents a single symbol.
/// - `SymbolSet(SymbolSet)`: Represents a set of symbols.
pub enum MatchableSymbol {
    Symbol(Symbol),
    SymbolSet(SymbolSet),
}

impl MatchableSymbol {
    pub fn as_symbol(&self) -> Option<&Symbol> {
        if let Self::Symbol(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_symbol_set(&self) -> Option<&SymbolSet> {
        if let Self::SymbolSet(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Display for MatchableSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchableSymbol::Symbol(symbol) => write!(f, "{}", symbol),
            MatchableSymbol::SymbolSet(symbol_set) => match symbol_set {
                SymbolSet::CharSet(char_set) => {
                    write!(f, "{}", char_set)
                }
                SymbolSet::Dot => write!(f, "."),
            },
        }
    }
}

impl PartialEq<char> for MatchableSymbol {
    fn eq(&self, other: &char) -> bool {
        match self {
            MatchableSymbol::Symbol(symbol) => symbol == other,
            MatchableSymbol::SymbolSet(symbol_set) => symbol_set == other,
        }
    }
}

impl PartialEq<char> for SymbolSet {
    fn eq(&self, other: &char) -> bool {
        match self {
            SymbolSet::CharSet(set) => set == other,
            SymbolSet::Dot => true, // Dot matches any character
        }
    }
}

impl PartialEq<char> for Symbol {
    fn eq(&self, other: &char) -> bool {
        match self {
            Symbol::Char(c) => c == other,
            Symbol::Epsilon => false, // Epsilon does not match any character
        }
    }
}
