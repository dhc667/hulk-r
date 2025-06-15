use crate::error::error::{HulkError, HulkErrorTrait};
use crate::error::semantic::semantic_error::SemanticError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidReassigmentTarget {
    pub target: String,
    pub position: usize,
}

impl InvalidReassigmentTarget {
    pub fn new(target: String, position: usize) -> Self {
        Self { target, position }
    }
}

impl fmt::Display for InvalidReassigmentTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not a valid reassignment target.", self.target)
    }
}

impl HulkErrorTrait for InvalidReassigmentTarget {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<InvalidReassigmentTarget> for HulkError {
    fn from(e: InvalidReassigmentTarget) -> Self {
        HulkError::SemanticError(SemanticError::InvalidReassigmentTarget(e))
    }
}

#[derive(Debug, Clone)]
pub struct InvalidReassignmentType {
    pub name: String,
    pub expected: String,
    pub got: String,
    pub position: usize,
}

impl InvalidReassignmentType {
    pub fn new(name: String, expected: String, got: String, position: usize) -> Self {
        Self {
            name,
            expected,
            got,
            position,
        }
    }
}

impl fmt::Display for InvalidReassignmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` has type `{}` but is being reassigned with type `{}`.",
            self.name, self.expected, self.got
        )
    }
}

impl HulkErrorTrait for InvalidReassignmentType {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<InvalidReassignmentType> for HulkError {
    fn from(e: InvalidReassignmentType) -> Self {
        HulkError::SemanticError(SemanticError::InvalidReassignmentType(e))
    }
}

#[derive(Debug, Clone)]
pub struct ListInvalidReassignmentType {
    pub from: String,
    pub to: String,
    pub position: usize,
}

impl ListInvalidReassignmentType {
    pub fn new(from: String, to: String, position: usize) -> Self {
        Self { from, to, position }
    }
}

impl fmt::Display for ListInvalidReassignmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cannot assign `{}` to list element of type `{}`.",
            self.from, self.to
        )
    }
}

impl HulkErrorTrait for ListInvalidReassignmentType {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<ListInvalidReassignmentType> for HulkError {
    fn from(e: ListInvalidReassignmentType) -> Self {
        HulkError::SemanticError(SemanticError::ListInvalidReassignmentType(e))
    }
}

#[derive(Debug, Clone)]
pub struct InvalidReassigmentExpression {
    pub position: usize,
}

impl InvalidReassigmentExpression {
    pub fn new(position: usize) -> Self {
        Self { position }
    }
}

impl fmt::Display for InvalidReassigmentExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Only variables and self properties can be assigned.")
    }
}

impl HulkErrorTrait for InvalidReassigmentExpression {
    fn get_position(&self) -> usize {
        self.position
    }
}

impl From<InvalidReassigmentExpression> for HulkError {
    fn from(e: InvalidReassigmentExpression) -> Self {
        HulkError::SemanticError(SemanticError::InvalidReassigmentExpression(e))
    }
}
