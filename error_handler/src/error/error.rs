use std::fmt::Display;

use crate::error::{
    lexical::lexical_error::LexicalError, semantic::semantic_error::SemanticError,
    sintactic::syntactic_error::SyntacticError,
};

#[derive(Debug, Clone)]
pub enum HulkError {
    LexicalError(LexicalError),
    SyntacticError(SyntacticError),
    SemanticError(SemanticError),
}

pub trait HulkErrorTrait: Display + Into<HulkError> {
    fn get_position(&self) -> usize;
}

impl Display for HulkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HulkError::LexicalError(err) => write!(f, "{}", err),
            HulkError::SyntacticError(err) => write!(f, "{}", err),
            HulkError::SemanticError(err) => write!(f, "{}", err),
        }
    }
}

impl HulkErrorTrait for HulkError {
    fn get_position(&self) -> usize {
        match self {
            HulkError::LexicalError(err) => err.get_position(),
            HulkError::SyntacticError(err) => err.get_position(),
            HulkError::SemanticError(err) => err.get_position(),
        }
    }
}
