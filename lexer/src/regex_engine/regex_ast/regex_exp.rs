use std::fmt::Display;

use crate::regex_engine::regex_ast::{bin_op::BinOp, symbol::symbol::MatchableSymbol, un_op::UnOp};

/// Represents a regular expression expression, which can be an atom (a matchable symbol),
/// a binary operation, or a unary operation.
/// # Variants
/// - `Atom`: Represents a matchable symbol, such as a character or a set of characters.
/// - `BinOp`: Represents a binary operation, such as concatenation or alternation.
/// - `UnOp`: Represents a unary operation, such as Kleene star, plus, or optional.
pub enum RegexExp {
    Atom(MatchableSymbol),
    BinOp(BinOp),
    UnOp(UnOp),
}

impl RegexExp {
    pub fn as_atom(&self) -> Option<&MatchableSymbol> {
        if let Self::Atom(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_bin_op(&self) -> Option<&BinOp> {
        if let Self::BinOp(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_un_op(&self) -> Option<&UnOp> {
        if let Self::UnOp(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Display for RegexExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => write!(f, "{}", atom),
            Self::BinOp(bin_op) => write!(f, "{}", bin_op),
            Self::UnOp(un_op) => write!(f, "{}", un_op),
        }
    }
}
