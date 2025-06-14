use std::fmt::Display;

use crate::error::{
    error::{HulkError, HulkErrorTrait},
    sintactic::syntactic_error::SyntacticError,
};

pub struct UnrecognizedTokenError {
    token: String,
    expected: Vec<String>,
    position: usize,
}

impl UnrecognizedTokenError {
    pub fn new(token: String, expected: Vec<String>, position: usize) -> Self {
        Self {
            token,
            expected,
            position,
        }
    }
}

impl Display for UnrecognizedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expected_clean: Vec<String> = self
            .expected
            .iter()
            .map(|s| s.replace('"', "`").replace("\\", ""))
            .collect();
        let expected_str = expected_clean.join(", ");
        write!(
            f,
            "Unrecognized token `{}` found, expected: {}",
            self.token, expected_str
        )
    }
}

impl Into<HulkError> for UnrecognizedTokenError {
    fn into(self) -> HulkError {
        HulkError::SyntacticError(SyntacticError::UnrecognizedToken(self))
    }
}

impl HulkErrorTrait for UnrecognizedTokenError {
    fn get_position(&self) -> usize {
        self.position
    }
}
