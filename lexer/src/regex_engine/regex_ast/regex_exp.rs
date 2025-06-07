use std::fmt::Display;

use crate::regex_engine::regex_ast::{bin_op::BinOp, symbol::symbol::MatchableSymbol, un_op::UnOp};

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
