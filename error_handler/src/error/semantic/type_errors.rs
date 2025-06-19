use crate::error::{
    error::{HulkError, HulkErrorTrait},
    semantic::semantic_error::SemanticError,
};

#[derive(Debug, Clone)]
pub struct NeedsAnAnnotation {
    name: String,
    position: usize,
}

impl NeedsAnAnnotation {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl std::fmt::Display for NeedsAnAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type of `{}` could not be resolved", self.name)
    }
}

impl HulkErrorTrait for NeedsAnAnnotation {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<NeedsAnAnnotation> for HulkError {
    fn from(e: NeedsAnAnnotation) -> Self {
        HulkError::SemanticError(SemanticError::NeedsAnAnnotation(e))
    }
}

#[derive(Debug, Clone)]
pub struct UnknownListType {
    position: usize,
}

impl UnknownListType {
    pub fn new(position: usize) -> Self {
        Self { position }
    }
}

impl std::fmt::Display for UnknownListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown type for list")
    }
}

impl HulkErrorTrait for UnknownListType {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<UnknownListType> for HulkError {
    fn from(e: UnknownListType) -> Self {
        HulkError::SemanticError(SemanticError::UnknownListType(e))
    }
}

#[derive(Debug, Clone)]
pub struct NeedsMoreSpecificType {
    name: String,
    received: String,
    position: usize,
}

impl NeedsMoreSpecificType {
    pub fn new(name: String, received: String, position: usize) -> Self {
        Self {
            name,
            received,
            position,
        }
    }
}

impl std::fmt::Display for NeedsMoreSpecificType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Variable `{}` needs a more specific type than `{}`",
            self.name, self.received
        )
    }
}

impl HulkErrorTrait for NeedsMoreSpecificType {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<NeedsMoreSpecificType> for HulkError {
    fn from(e: NeedsMoreSpecificType) -> Self {
        HulkError::SemanticError(SemanticError::NeedsMoreSpecificType(e))
    }
}
