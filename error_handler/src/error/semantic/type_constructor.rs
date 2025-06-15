use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TypeParamsInvalidAmount {
    pub name: String,
    pub expected: usize,
    pub provided: usize,
    pub position: usize,
}

impl TypeParamsInvalidAmount {
    pub fn new(name: String, expected: usize, provided: usize, position: usize) -> Self {
        Self {
            name,
            expected,
            provided,
            position,
        }
    }
}

impl fmt::Display for TypeParamsInvalidAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type `{}` has {} parameters, but {} were provided.",
            self.name, self.expected, self.provided
        )
    }
}

impl HulkErrorTrait for TypeParamsInvalidAmount {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<TypeParamsInvalidAmount> for HulkError {
    fn from(e: TypeParamsInvalidAmount) -> Self {
        HulkError::SemanticError(SemanticError::TypeParamsInvalidAmount(e))
    }
}

#[derive(Debug, Clone)]
pub struct TypeParamInvalidType {
    pub name: String,
    pub param: String,
    pub expected: String,
    pub got: String,
    pub position: usize,
}

impl TypeParamInvalidType {
    pub fn new(
        name: String,
        param: String,
        expected: String,
        got: String,
        position: usize,
    ) -> Self {
        Self {
            name,
            param,
            expected,
            got,
            position,
        }
    }
}

impl fmt::Display for TypeParamInvalidType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type `{}` expects parameter `{}` of type `{}`, but got {}.",
            self.name, self.param, self.expected, self.got
        )
    }
}

impl HulkErrorTrait for TypeParamInvalidType {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<TypeParamInvalidType> for HulkError {
    fn from(e: TypeParamInvalidType) -> Self {
        HulkError::SemanticError(SemanticError::TypeParamInvalidType(e))
    }
}
