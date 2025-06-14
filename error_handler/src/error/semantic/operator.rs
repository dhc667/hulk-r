use std::fmt::Display;

use crate::error::{
    error::{HulkError, HulkErrorTrait},
    semantic::semantic_error::SemanticError,
};

pub struct BinOpError {
    operator: String,
    left_type: String,
    right_type: String,
    position: usize,
}

impl BinOpError {
    pub fn new(operator: String, left_type: String, right_type: String, position: usize) -> Self {
        BinOpError {
            operator,
            left_type,
            right_type,
            position,
        }
    }
}

impl Display for BinOpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cannot apply `{}` to operands of type `{}` and `{}`.",
            self.operator, self.left_type, self.right_type
        )
    }
}

impl Into<HulkError> for BinOpError {
    fn into(self) -> HulkError {
        HulkError::SemanticError(SemanticError::BinOpInvalidOperands(self))
    }
}

impl HulkErrorTrait for BinOpError {
    fn get_position(&self) -> usize {
        self.position
    }
}

pub struct UnOpError {
    operator: String,
    operand_type: String,
    position: usize,
}

impl UnOpError {
    pub fn new(operator: String, operand_type: String, position: usize) -> Self {
        UnOpError {
            operator,
            operand_type,
            position,
        }
    }
}

impl Display for UnOpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cannot apply `{}` to operand of type `{}`.",
            self.operator, self.operand_type
        )
    }
}
impl Into<HulkError> for UnOpError {
    fn into(self) -> HulkError {
        HulkError::SemanticError(SemanticError::UnOpInvalidOperands(self))
    }
}

impl HulkErrorTrait for UnOpError {
    fn get_position(&self) -> usize {
        self.position
    }
}
