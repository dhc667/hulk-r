use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct InheritanceInvalidParent {
    pub name: String,
    pub position: usize,
}

impl InheritanceInvalidParent {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl fmt::Display for InheritanceInvalidParent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type `{}` is a built-in type and cannot be inherited from.",
            self.name
        )
    }
}

impl HulkErrorTrait for InheritanceInvalidParent {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<InheritanceInvalidParent> for HulkError {
    fn from(e: InheritanceInvalidParent) -> Self {
        HulkError::SemanticError(SemanticError::InheritanceInvalidParent(e))
    }
}

#[derive(Debug, Clone)]
pub struct InheritanceCycle {
    cycle: Vec<String>,
    position: usize,
}

impl InheritanceCycle {
    pub fn new(cycle: Vec<String>, position: usize) -> Self {
        Self { cycle, position }
    }
}

impl fmt::Display for InheritanceCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Inheritance cycle detected: {}.",
            self.cycle.join(" -> ")
        )
    }
}

impl HulkErrorTrait for InheritanceCycle {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<InheritanceCycle> for HulkError {
    fn from(e: InheritanceCycle) -> Self {
        HulkError::SemanticError(SemanticError::InheritanceCycle(e))
    }
}

#[derive(Debug, Clone)]
pub struct InvalidIfElseType {
    pub position: usize,
}

impl InvalidIfElseType {
    pub fn new(position: usize) -> Self {
        Self { position }
    }
}

impl Display for InvalidIfElseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "If-else expresssion must have a more specific type than `Object`.",
        )
    }
}

impl From<InvalidIfElseType> for HulkError {
    fn from(e: InvalidIfElseType) -> Self {
        HulkError::SemanticError(SemanticError::InvalidIfElseType(e))
    }
}

impl HulkErrorTrait for InvalidIfElseType {
    fn get_position(&self) -> usize {
        self.position
    }
}

#[derive(Debug, Clone)]
pub struct InvalidListLiteralType {
    pub position: usize,
}

impl InvalidListLiteralType {
    pub fn new(position: usize) -> Self {
        Self { position }
    }
}

impl Display for InvalidListLiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "List literal must have a more specific type than `Object`."
        )
    }
}

impl From<InvalidListLiteralType> for HulkError {
    fn from(e: InvalidListLiteralType) -> Self {
        HulkError::SemanticError(SemanticError::InvalidListLiteralType(e))
    }
}

impl HulkErrorTrait for InvalidListLiteralType {
    fn get_position(&self) -> usize {
        self.position
    }
}

#[derive(Debug, Clone)]
pub struct ObjectAnnotationError {
    pub position: usize,
}

impl ObjectAnnotationError {
    pub fn new(position: usize) -> Self {
        Self { position }
    }
}

impl Display for ObjectAnnotationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Annotations must be of a more specific type than `Object`."
        )
    }
}

impl From<ObjectAnnotationError> for HulkError {
    fn from(e: ObjectAnnotationError) -> Self {
        HulkError::SemanticError(SemanticError::ObjectAnnotationError(e))
    }
}

impl HulkErrorTrait for ObjectAnnotationError {
    fn get_position(&self) -> usize {
        self.position
    }
}
