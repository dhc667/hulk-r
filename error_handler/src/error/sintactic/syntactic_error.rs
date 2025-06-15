use std::fmt::Display;

use crate::error::{
    error::{HulkError, HulkErrorTrait},
    sintactic::{
        extra_token::ExtraTokenError, invalid_token::InvalidTokenError,
        unrecognized_eof::UnrecognizedEofError, unrecognized_token::UnrecognizedTokenError,
        user_error::UserError,
    },
};

#[derive(Debug, Clone)]
pub enum SyntacticError {
    InvalidToken(InvalidTokenError),
    UnrecognizedEof(UnrecognizedEofError),
    UnrecognizedToken(UnrecognizedTokenError),
    ExtraToken(ExtraTokenError),
    UserError(UserError),
}

impl HulkErrorTrait for SyntacticError {
    fn get_position(&self) -> usize {
        match self {
            SyntacticError::InvalidToken(err) => err.get_position(),
            SyntacticError::UnrecognizedEof(err) => err.get_position(),
            SyntacticError::UnrecognizedToken(err) => err.get_position(),
            SyntacticError::ExtraToken(err) => err.get_position(),
            SyntacticError::UserError(err) => err.get_position(),
        }
    }
}

impl Into<HulkError> for SyntacticError {
    fn into(self) -> HulkError {
        HulkError::SyntacticError(self)
    }
}
impl Display for SyntacticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntacticError::InvalidToken(err) => write!(f, "Sintactic Error: {}", err),
            SyntacticError::UnrecognizedEof(err) => write!(f, "Sintactic Error: {}", err),
            SyntacticError::UnrecognizedToken(err) => write!(f, "Sintactic Error: {}", err),
            SyntacticError::ExtraToken(err) => write!(f, "Sintactic Error: {}", err),
            SyntacticError::UserError(err) => write!(f, "Sintactic Error: {}", err),
        }
    }
}
