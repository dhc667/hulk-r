use std::fmt::Display;

use crate::error::{
    error::{HulkError, HulkErrorTrait},
    lexical::lexical_error::LexicalError,
};

pub struct InvalidCharacterError {
    character: char,
    position: usize,
}

impl InvalidCharacterError {
    pub fn new(character: char, position: usize) -> Self {
        Self {
            character,
            position,
        }
    }
}

impl HulkErrorTrait for InvalidCharacterError {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl Into<HulkError> for InvalidCharacterError {
    fn into(self) -> HulkError {
        HulkError::LexicalError(LexicalError::InvalidCharacter(self))
    }
}

impl Display for InvalidCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid character `{}`", self.character)
    }
}
