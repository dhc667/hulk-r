use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UndefinedVariable {
    pub name: String,
    pub position: usize,
}

impl UndefinedVariable {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl fmt::Display for UndefinedVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Variable `{}` is not defined.", self.name)
    }
}

impl HulkErrorTrait for UndefinedVariable {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<UndefinedVariable> for HulkError {
    fn from(e: UndefinedVariable) -> Self {
        HulkError::SemanticError(SemanticError::UndefinedVariable(e))
    }
}

#[derive(Debug, Clone)]
pub struct UndefinedFunction {
    pub name: String,
    pub position: usize,
}

impl UndefinedFunction {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl fmt::Display for UndefinedFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function `{}` is not defined.", self.name)
    }
}

impl HulkErrorTrait for UndefinedFunction {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<UndefinedFunction> for HulkError {
    fn from(e: UndefinedFunction) -> Self {
        HulkError::SemanticError(SemanticError::UndefinedFunction(e))
    }
}

#[derive(Debug, Clone)]
pub struct UndefinedType {
    pub name: String,
    pub position: usize,
}

impl UndefinedType {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl fmt::Display for UndefinedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type `{}` is not defined.", self.name)
    }
}

impl HulkErrorTrait for UndefinedType {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<UndefinedType> for HulkError {
    fn from(e: UndefinedType) -> Self {
        HulkError::SemanticError(SemanticError::UndefinedType(e))
    }
}

#[derive(Debug, Clone)]
pub struct UndefinedTypeOrProtocol {
    pub name: String,
    pub position: usize,
}

impl UndefinedTypeOrProtocol {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl fmt::Display for UndefinedTypeOrProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type or protocol `{}` is not defined.", self.name)
    }
}

impl HulkErrorTrait for UndefinedTypeOrProtocol {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<UndefinedTypeOrProtocol> for HulkError {
    fn from(e: UndefinedTypeOrProtocol) -> Self {
        HulkError::SemanticError(SemanticError::UndefinedTypeOrProtocol(e))
    }
}
