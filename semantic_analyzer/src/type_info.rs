use std::collections::HashMap;
use std::fmt::Display;

use ast::{TypeName, typing::BuiltInType};
use std::fmt::{Formatter, Result};

use crate::DefinitionInfo;

pub enum TypeInfo {
    BuiltIn(BuiltInType),
    Defined(DefinedTypeInfo),
}

pub struct DefinedTypeInfo {
    pub name: TypeName,
    pub members: HashMap<String, DefinitionInfo>, // TODO: Add functions
}

impl DefinedTypeInfo {
    pub fn new(name: TypeName, members: HashMap<String, DefinitionInfo>) -> Self {
        DefinedTypeInfo { name, members }
    }
}

impl Display for DefinedTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name.id)
    }
}
