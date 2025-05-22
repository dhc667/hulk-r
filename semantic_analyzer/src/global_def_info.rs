use crate::{DefinitionInfo, FuncInfo, TypeInfo};

#[derive(Clone)]
pub enum GlobalDefinitionInfo {
    Var(DefinitionInfo),
    Type(TypeInfo),
    Func(FuncInfo),
}

impl GlobalDefinitionInfo {
    pub fn as_type(&self) -> Option<&TypeInfo> {
        if let Self::Type(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_func(&self) -> Option<&FuncInfo> {
        if let Self::Func(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_var(&self) -> Option<&DefinitionInfo> {
        if let Self::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
