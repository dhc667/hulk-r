use super::{FuncInfo, TypeInfo, VarInfo};

/// # Description
/// `DefinitionInfo` is an enum that encapsulates different types of definitions
/// in the semantic analyzer.
/// It can represent a variable, a type, or a function definition.
/// # Variants
/// - `Var`: Represents a variable definition, encapsulated in `VarInfo`.
/// - `Type`: Represents a type definition, encapsulated in `TypeInfo`.
/// - `Func`: Represents a function definition, encapsulated in `FuncInfo`.
#[derive(Clone)]
pub enum DefinitionInfo {
    Var(VarInfo),
    Type(TypeInfo),
    Func(FuncInfo),
}

impl DefinitionInfo {
    pub fn as_type(&self) -> Option<&TypeInfo> {
        if let Self::Type(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_type_mut(&mut self) -> Option<&mut TypeInfo> {
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

    pub fn as_func_mut(&mut self) -> Option<&mut FuncInfo> {
        if let Self::Func(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_var(&self) -> Option<&VarInfo> {
        if let Self::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_var_mut(&mut self) -> Option<&mut VarInfo> {
        if let Self::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
