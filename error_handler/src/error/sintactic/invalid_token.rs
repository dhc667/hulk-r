use std::fmt::Display;

use crate::error::{
    error::{HulkError, HulkErrorTrait},
    sintactic::syntactic_error::SyntacticError,
};

pub struct InvalidTokenError {
    position: usize,
}

impl InvalidTokenError {
    pub fn new(position: usize) -> Self {
        Self { position }
    }
}

impl Into<HulkError> for InvalidTokenError {
    fn into(self) -> HulkError {
        HulkError::SyntacticError(SyntacticError::InvalidToken(self))
    }
}

impl Display for InvalidTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid token found")
    }
}

impl HulkErrorTrait for InvalidTokenError {
    fn get_position(&self) -> usize {
        self.position
    }
}
