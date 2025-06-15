use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TypeOrProtocolAlreadyDefined {
    pub name: String,
    pub position: usize,
}

impl TypeOrProtocolAlreadyDefined {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl fmt::Display for TypeOrProtocolAlreadyDefined {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Already exists a type or protocol `{}`.", self.name)
    }
}

impl HulkErrorTrait for TypeOrProtocolAlreadyDefined {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<TypeOrProtocolAlreadyDefined> for HulkError {
    fn from(e: TypeOrProtocolAlreadyDefined) -> Self {
        HulkError::SemanticError(SemanticError::TypeOrProtocolAlreadyDefined(e))
    }
}

#[derive(Debug, Clone)]
pub struct TypeMemberAlreadyDefined {
    pub member: String,
    pub type_name: String,
    pub position: usize,
}

impl TypeMemberAlreadyDefined {
    pub fn new(member: String, type_name: String, position: usize) -> Self {
        Self {
            member,
            type_name,
            position,
        }
    }
}

impl fmt::Display for TypeMemberAlreadyDefined {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Member `{}` is already defined in type `{}`.",
            self.member, self.type_name
        )
    }
}

impl HulkErrorTrait for TypeMemberAlreadyDefined {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<TypeMemberAlreadyDefined> for HulkError {
    fn from(e: TypeMemberAlreadyDefined) -> Self {
        HulkError::SemanticError(SemanticError::TypeMemberAlreadyDefined(e))
    }
}
