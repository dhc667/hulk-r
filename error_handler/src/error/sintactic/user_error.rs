use crate::error::{
    error::{HulkError, HulkErrorTrait},
    sintactic::syntactic_error::SyntacticError,
};

#[derive(Debug, Clone)]
pub struct UserError {
    pub message: String,
    pub position: usize,
}

impl UserError {
    pub fn new(message: String, position: usize) -> Self {
        Self { message, position }
    }
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<UserError> for HulkError {
    fn from(e: UserError) -> Self {
        HulkError::SyntacticError(SyntacticError::UserError(e))
    }
}

impl HulkErrorTrait for UserError {
    fn get_position(&self) -> usize {
        self.position
    }
}
