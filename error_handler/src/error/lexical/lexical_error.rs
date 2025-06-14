use crate::error::lexical::invalid_character::InvalidCharacterError;

pub enum LexicalError {
    InvalidCharacter(InvalidCharacterError),
}
