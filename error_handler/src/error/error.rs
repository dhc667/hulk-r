use crate::error::lexical::lexical_error::LexicalError;

pub enum HulkError {
    LexicalError(LexicalError),
    SyntacticError(),
    SemanticError(String),
}

pub trait HulkErrorTrait {
    fn get_position(&self) -> usize;
}
