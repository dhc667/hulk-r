use std::fmt::Display;

use crate::error::{
    lexical::lexical_error::LexicalError, semantic::semantic_error::SemanticError,
    sintactic::syntactic_error::SyntacticError,
};

pub enum HulkError {
    LexicalError(LexicalError),
    SyntacticError(SyntacticError),
    SemanticError(SemanticError),
}

pub trait HulkErrorTrait: Display + Into<HulkError> {
    fn get_position(&self) -> usize;
}
