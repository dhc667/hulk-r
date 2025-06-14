use std::fmt::Display;

use crate::error::{
    error::{HulkError, HulkErrorTrait},
    sintactic::syntactic_error::SyntacticError,
};

#[derive(Debug, Clone)]
pub struct ExtraTokenError {
    token: String,
    position: usize,
}

impl ExtraTokenError {
    pub fn new(token: String, position: usize) -> Self {
        Self { token, position }
    }
}

impl HulkErrorTrait for ExtraTokenError {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl Into<HulkError> for ExtraTokenError {
    fn into(self) -> crate::error::error::HulkError {
        HulkError::SyntacticError(SyntacticError::ExtraToken(self))
    }
}

impl Display for ExtraTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Extra token `{}` found", self.token)
    }
}
