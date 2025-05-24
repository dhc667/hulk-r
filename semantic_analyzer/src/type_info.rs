use std::collections::HashMap;
use std::fmt::Display;

use ast::{
    TypeName,
    typing::{BuiltInType, Type, TypeAnnotation},
};
use std::fmt::{Formatter, Result};

use crate::GlobalDefinitionInfo;

#[derive(Clone)]
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

    pub fn as_built_in_mut(&mut self) -> Option<&mut BuiltInType> {
        if let Self::BuiltIn(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_defined_mut(&mut self) -> Option<&mut DefinedTypeInfo> {
        if let Self::Defined(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_type_annotation(&self) -> TypeAnnotation {
        match self {
            TypeInfo::Defined(ty) => Some(Type::Defined(ty.name.clone())),
            TypeInfo::BuiltIn(ty) => Some(Type::BuiltIn(ty.clone())),
        }
    }
}

#[derive(Clone)]
pub struct DefinedTypeInfo {
    pub name: TypeName,
    pub members: HashMap<String, GlobalDefinitionInfo>,
    pub arguments_types: Vec<TypeAnnotation>,
}

impl DefinedTypeInfo {
    pub fn new(
        name: TypeName,
        members: HashMap<String, GlobalDefinitionInfo>,
        arguments_types: Vec<TypeAnnotation>,
    ) -> Self {
        DefinedTypeInfo {
            name,
            members,
            arguments_types,
        }
    }
}

impl Display for DefinedTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name.id)
    }
}
