use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FieldOverride {
    pub field: String,
    pub type_name: String,
    pub position: usize,
}

impl fmt::Display for FieldOverride {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cannot declare field `{}` in type `{}`, as it overrides parent definition.",
            self.field, self.type_name
        )
    }
}

impl HulkErrorTrait for FieldOverride {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<FieldOverride> for HulkError {
    fn from(e: FieldOverride) -> Self {
        HulkError::SemanticError(SemanticError::FieldOverride(e))
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMethodOverride {
    pub method: String,
    pub type_name: String,
    pub position: usize,
}

impl fmt::Display for InvalidMethodOverride {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Method `{}` in type `{}`, does not properly overrides parent definition.",
            self.method, self.type_name
        )
    }
}

impl HulkErrorTrait for InvalidMethodOverride {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<InvalidMethodOverride> for HulkError {
    fn from(e: InvalidMethodOverride) -> Self {
        HulkError::SemanticError(SemanticError::InvalidMethodOverride(e))
    }
}
