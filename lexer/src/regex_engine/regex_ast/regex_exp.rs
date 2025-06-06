use crate::regex_engine::regex_ast::{bin_op::BinOp, symbol::Symbol, un_op::UnOp};

pub enum RegexExp {
    Symbol(Symbol),
    BinOp(BinOp),
    UnOp(UnOp),
}

impl RegexExp {
    pub fn as_symbol(&self) -> Option<&Symbol> {
        if let Self::Symbol(v) = self {
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
