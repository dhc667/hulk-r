use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FuncAlreadyDefined {
    name: String,
    position: usize,
}

impl FuncAlreadyDefined {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl fmt::Display for FuncAlreadyDefined {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function `{}` is already defined.", self.name)
    }
}

impl HulkErrorTrait for FuncAlreadyDefined {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<FuncAlreadyDefined> for HulkError {
    fn from(e: FuncAlreadyDefined) -> Self {
        HulkError::SemanticError(SemanticError::FuncAlreadyDefined(e))
    }
}

#[derive(Debug, Clone)]
pub struct FuncParamsInvalidAmount {
    name: String,
    expected: usize,
    provided: usize,
    position: usize,
}

impl FuncParamsInvalidAmount {
    pub fn new(name: String, expected: usize, provided: usize, position: usize) -> Self {
        Self {
            name,
            expected,
            provided,
            position,
        }
    }
}

impl fmt::Display for FuncParamsInvalidAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Function `{}` expects {} parameters, but {} were provided.",
            self.name, self.expected, self.provided
        )
    }
}

impl HulkErrorTrait for FuncParamsInvalidAmount {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<FuncParamsInvalidAmount> for HulkError {
    fn from(e: FuncParamsInvalidAmount) -> Self {
        HulkError::SemanticError(SemanticError::FuncParamsInvalidAmount(e))
    }
}

#[derive(Debug, Clone)]
pub struct FuncParamInvalidType {
    pub name: String,
    pub param: String,
    pub expected: String,
    pub got: String,
    pub position: usize,
}

impl FuncParamInvalidType {
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

impl fmt::Display for FuncParamInvalidType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Function `{}` expects parameter `{}` of type `{}`, but got `{}`.",
            self.name, self.param, self.expected, self.got
        )
    }
}

impl HulkErrorTrait for FuncParamInvalidType {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<FuncParamInvalidType> for HulkError {
    fn from(e: FuncParamInvalidType) -> Self {
        HulkError::SemanticError(SemanticError::FuncParamInvalidType(e))
    }
}

#[derive(Debug, Clone)]
pub struct FuncReturnTypeInvalid {
    name: String,
    expected: String,
    found: String,
    position: usize,
}

impl FuncReturnTypeInvalid {
    pub fn new(name: String, expected: String, found: String, position: usize) -> Self {
        Self {
            name,
            expected,
            found,
            position,
        }
    }
}

impl fmt::Display for FuncReturnTypeInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Function `{}` returns `{}` but `{}` was found.",
            self.name, self.expected, self.found
        )
    }
}

impl HulkErrorTrait for FuncReturnTypeInvalid {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<FuncReturnTypeInvalid> for HulkError {
    fn from(e: FuncReturnTypeInvalid) -> Self {
        HulkError::SemanticError(SemanticError::FuncReturnTypeInvalid(e))
    }
}
