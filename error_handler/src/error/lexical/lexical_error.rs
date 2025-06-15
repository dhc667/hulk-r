use std::fmt::{Display, Formatter};

use crate::error::{
    error::{HulkError, HulkErrorTrait},
    lexical::invalid_character::InvalidCharacterError,
};

#[derive(Debug, Clone)]
pub enum LexicalError {
    InvalidCharacter(InvalidCharacterError),
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexicalError::InvalidCharacter(err) => write!(f, "Lexical Error: {}", err),
        }
    }
}

impl Into<HulkError> for LexicalError {
    fn into(self) -> HulkError {
        HulkError::LexicalError(self)
    }
}

impl HulkErrorTrait for LexicalError {
    fn get_position(&self) -> usize {
        match self {
            LexicalError::InvalidCharacter(err) => err.get_position(),
        }
    }
}
