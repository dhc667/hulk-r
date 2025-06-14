use std::fmt::Display;

use crate::error::{
    error::{HulkError, HulkErrorTrait},
    sintactic::syntactic_error::SyntacticError,
};

pub struct UnrecognizedEofError {
    expected: Vec<String>,
    position: usize,
}

impl UnrecognizedEofError {
    pub fn new(expected: Vec<String>, position: usize) -> Self {
        Self { expected, position }
    }
}

impl HulkErrorTrait for UnrecognizedEofError {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl Into<HulkError> for UnrecognizedEofError {
    fn into(self) -> HulkError {
        HulkError::SyntacticError(SyntacticError::UnrecognizedEof(self))
    }
}

impl Display for UnrecognizedEofError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expected_clean: Vec<String> = self
            .expected
            .iter()
            .map(|s| s.replace('"', "`").replace("\\", ""))
            .collect();
        let expected_str = expected_clean.join(", ");
        write!(f, "Unrecognized EOF found, expected: {}", expected_str)
    }
}
