use std::collections::HashMap;
use std::fmt::Display;

use ast::{TypeName, typing::BuiltInType};
use std::fmt::{Formatter, Result};

use crate::{DefinitionInfo, FuncInfo};

pub enum TypeInfo {
    BuiltIn(BuiltInType),
    Defined(DefinedTypeInfo),
}

impl TypeInfo {
    pub fn as_built_in(&self) -> Option<&BuiltInType> {
        if let Self::BuiltIn(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_defined(&self) -> Option<&DefinedTypeInfo> {
        if let Self::Defined(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

pub struct DefinedTypeInfo {
    pub name: TypeName,
    pub members: HashMap<String, DefinitionInfo>,
    pub methods: HashMap<String, FuncInfo>,
}

impl DefinedTypeInfo {
    pub fn new(
        name: TypeName,
        members: HashMap<String, DefinitionInfo>,
        methods: HashMap<String, FuncInfo>,
    ) -> Self {
        DefinedTypeInfo {
            name,
            members,
            methods,
        }
    }
}

impl Display for DefinedTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name.id)
    }
}
