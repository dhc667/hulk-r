use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VarDefinitionTypeMismatch {
    pub from: String,
    pub to: String,
    pub position: usize,
}

impl VarDefinitionTypeMismatch {
    pub fn new(from: String, to: String, position: usize) -> Self {
        Self { from, to, position }
    }
}

impl fmt::Display for VarDefinitionTypeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot assign `{}` to `{}`.", self.from, self.to)
    }
}

impl HulkErrorTrait for VarDefinitionTypeMismatch {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<VarDefinitionTypeMismatch> for HulkError {
    fn from(e: VarDefinitionTypeMismatch) -> Self {
        HulkError::SemanticError(SemanticError::VarDefinitionTypeMismatch(e))
    }
}

#[derive(Debug, Clone)]
pub struct VarAlreadyDefined {
    pub name: String,
    pub position: usize,
}

impl VarAlreadyDefined {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl fmt::Display for VarAlreadyDefined {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Constant `{}` is already defined.", self.name)
    }
}

impl HulkErrorTrait for VarAlreadyDefined {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<VarAlreadyDefined> for HulkError {
    fn from(e: VarAlreadyDefined) -> Self {
        HulkError::SemanticError(SemanticError::VarAlreadyDefined(e))
    }
}
