use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AccessingPrivateMember {
    pub member: String,
    pub type_name: String,
    pub position: usize,
}

impl fmt::Display for AccessingPrivateMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cannot access member `{}` of type `{}`. Properties are private, even to inherited types.",
            self.member, self.type_name
        )
    }
}

impl HulkErrorTrait for AccessingPrivateMember {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<AccessingPrivateMember> for HulkError {
    fn from(e: AccessingPrivateMember) -> Self {
        HulkError::SemanticError(SemanticError::AccessingPrivateMember(e))
    }
}

#[derive(Debug, Clone)]
pub struct FieldNotFound {
    pub member: String,
    pub position: usize,
}

impl fmt::Display for FieldNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not find data member `{}`.", self.member)
    }
}

impl HulkErrorTrait for FieldNotFound {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<FieldNotFound> for HulkError {
    fn from(e: FieldNotFound) -> Self {
        HulkError::SemanticError(SemanticError::FieldNotFound(e))
    }
}

#[derive(Debug, Clone)]
pub struct MethodNotFound {
    pub method: String,
    pub position: usize,
}

impl fmt::Display for MethodNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not find method `{}`.", self.method)
    }
}

impl HulkErrorTrait for MethodNotFound {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<MethodNotFound> for HulkError {
    fn from(e: MethodNotFound) -> Self {
        HulkError::SemanticError(SemanticError::MethodNotFound(e))
    }
}
