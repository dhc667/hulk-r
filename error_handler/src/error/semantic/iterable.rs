use crate::error::error::{HulkError, HulkErrorTrait};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct NonIterableType {
    pub type_name: String,
    pub position: usize,
}

impl NonIterableType {
    pub fn new(type_name: String, position: usize) -> Self {
        Self {
            type_name,
            position,
        }
    }
}

impl Display for NonIterableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot iterate over type `{}`.", self.type_name)
    }
}

impl HulkErrorTrait for NonIterableType {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<NonIterableType> for HulkError {
    fn from(err: NonIterableType) -> Self {
        HulkError::SemanticError(
            crate::error::semantic::semantic_error::SemanticError::NonIterableType(err),
        )
    }
}

#[derive(Debug, Clone)]
pub struct InvalidIndexing {
    pub index_type: String,
    pub position: usize,
}

impl InvalidIndexing {
    pub fn new(index_type: String, position: usize) -> Self {
        Self {
            index_type,
            position,
        }
    }
}

impl Display for InvalidIndexing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cannot use index of type `{}` to access iterable.",
            self.index_type
        )
    }
}

impl HulkErrorTrait for InvalidIndexing {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<InvalidIndexing> for HulkError {
    fn from(err: InvalidIndexing) -> Self {
        HulkError::SemanticError(
            crate::error::semantic::semantic_error::SemanticError::InvalidIndexing(err),
        )
    }
}
