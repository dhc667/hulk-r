use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InheritanceInvalidParent {
    pub name: String,
    pub position: usize,
}

impl fmt::Display for InheritanceInvalidParent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type `{}` is a built-in type and cannot be inherited from.",
            self.name
        )
    }
}

impl HulkErrorTrait for InheritanceInvalidParent {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<InheritanceInvalidParent> for HulkError {
    fn from(e: InheritanceInvalidParent) -> Self {
        HulkError::SemanticError(SemanticError::InheritanceInvalidParent(e))
    }
}

#[derive(Debug, Clone)]
pub struct InheritanceCycle {
    cycle: Vec<String>,
    position: usize,
}

impl fmt::Display for InheritanceCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Inheritance cycle detected: {}.",
            self.cycle.join(" -> ")
        )
    }
}

impl HulkErrorTrait for InheritanceCycle {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<InheritanceCycle> for HulkError {
    fn from(e: InheritanceCycle) -> Self {
        HulkError::SemanticError(SemanticError::InheritanceCycle(e))
    }
}
