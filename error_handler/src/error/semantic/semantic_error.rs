use std::fmt::Display;

use crate::error::{
    error::{HulkError, HulkErrorTrait},
    semantic::{
        definition::{
            UndefinedFunction, UndefinedType, UndefinedTypeOrProtocol, UndefinedVariable,
        },
        destructive_assignment::{
            InvalidReassigmentExpression, InvalidReassigmentTarget, InvalidReassignmentType,
            ListInvalidReassignmentType,
        },
        function::{
            FuncAlreadyDefined, FuncParamInvalidType, FuncParamsInvalidAmount,
            FuncReturnTypeInvalid,
        },
        inheritance::{InheritanceCycle, InheritanceInvalidParent},
        iterable::{InvalidIndexing, NonIterableType},
        member_access::{AccessingPrivateMember, FieldNotFound, MethodNotFound},
        operator::{BinOpError, UnOpError},
        override_error::{FieldOverride, InvalidMethodOverride},
        type_constructor::{TypeParamInvalidType, TypeParamsInvalidAmount},
        type_definition::{TypeMemberAlreadyDefined, TypeOrProtocolAlreadyDefined},
        variable_definition::{VarAlreadyDefined, VarDefinitionTypeMismatch},
    },
};

#[derive(Debug, Clone)]
pub enum SemanticError {
    // Type errors

    // operator
    BinOpInvalidOperands(BinOpError), // "Type mismatch: Cannot apply {} to operands of type {} and {}",
    UnOpInvalidOperands(UnOpError),   //"Type mismatch: Cannot apply {} to operand of type {}",

    // function
    FuncAlreadyDefined(FuncAlreadyDefined), // "Function {} is already defined",
    FuncParamsInvalidAmount(FuncParamsInvalidAmount), // "Function {} expects {} parameters, but {} were provided",
    FuncParamInvalidType(FuncParamInvalidType), // "Function {} expects parameter {} of type {}, but got {}",
    FuncReturnTypeInvalid(FuncReturnTypeInvalid), // "Type mismatch: Function {} returns {} but {} was found",

    // type constructor
    TypeParamsInvalidAmount(TypeParamsInvalidAmount), // "Type {} has {} parameters, but {} were provided",
    TypeParamInvalidType(TypeParamInvalidType), // "Type {} expects parameter {} of type {}, but got {}"

    // definition
    UndefinedVariable(UndefinedVariable), // "Variable {} is not defined"
    UndefinedFunction(UndefinedFunction), // "Function {} is not defined",w
    UndefinedType(UndefinedType),         // "Type {} is not defined",
    UndefinedTypeOrProtocol(UndefinedTypeOrProtocol), // "Semantic Error: Type or protocol {} is not defined.",

    // type definition
    TypeOrProtocolAlreadyDefined(TypeOrProtocolAlreadyDefined), // "Already exists a type or protocol {}"
    TypeMemberAlreadyDefined(TypeMemberAlreadyDefined), // "Member {} is already defined in type {}",

    // variable definition
    VarDefinitionTypeMismatch(VarDefinitionTypeMismatch), // "Type mismatch: Cannot assign {} to {}",
    VarAlreadyDefined(VarAlreadyDefined),                 // "Constant {} is already defined"

    // destructive assignment
    InvalidReassigmentTarget(InvalidReassigmentTarget), // "Semantic Error: `{}` is not a valid assignment target",
    InvalidReassignmentType(InvalidReassignmentType), // "Type mismatch: {} is {} but is being reassigned with {}",
    ListInvalidReassignmentType(ListInvalidReassignmentType), // "Type mismatch: Cannot assign {} to list element of type {}",
    InvalidReassigmentExpression(InvalidReassigmentExpression), // "Semantic Error: only variables and self properties can be assigned"

    // inheritance
    InheritanceInvalidParent(InheritanceInvalidParent), // "Type {} is a built-in type and cannot be inherited from",
    InheritanceCycle(InheritanceCycle), // "Semantic Error: Inheritance cycle detected

    // iterable
    NonIterableType(NonIterableType), // "Type mismatch: Cannot iterate over type {}",
    InvalidIndexing(InvalidIndexing), // "Type mismatch: Cannot use index of type {} to access iterable",

    // member access
    AccessingPrivateMember(AccessingPrivateMember), // "Cannot access member {} of type {}. Properties are private, even to inherited types.",
    FieldNotFound(FieldNotFound),                   // "Could not find data member {}"
    MethodNotFound(MethodNotFound),                 // "Could not find method {},

    // override
    FieldOverride(FieldOverride), // "Semantic Error: Cannot declare field {} in type {}, as it overrides parent definition.",
    InvalidMethodOverride(InvalidMethodOverride), // "Semantic Error: Method {} in type {}, does not properly overrides parent definition.",
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = match self {
            SemanticError::BinOpInvalidOperands(e) => format!("{}", e),
            SemanticError::UnOpInvalidOperands(e) => format!("{}", e),
            SemanticError::FuncAlreadyDefined(e) => format!("{}", e),
            SemanticError::FuncParamsInvalidAmount(e) => format!("{}", e),
            SemanticError::FuncParamInvalidType(e) => format!("{}", e),
            SemanticError::FuncReturnTypeInvalid(e) => format!("{}", e),
            SemanticError::TypeParamsInvalidAmount(e) => format!("{}", e),
            SemanticError::TypeParamInvalidType(e) => format!("{}", e),
            SemanticError::UndefinedVariable(e) => format!("{}", e),
            SemanticError::UndefinedFunction(e) => format!("{}", e),
            SemanticError::UndefinedType(e) => format!("{}", e),
            SemanticError::UndefinedTypeOrProtocol(e) => format!("{}", e),
            SemanticError::TypeOrProtocolAlreadyDefined(e) => format!("{}", e),
            SemanticError::TypeMemberAlreadyDefined(e) => format!("{}", e),
            SemanticError::VarDefinitionTypeMismatch(e) => format!("{}", e),
            SemanticError::VarAlreadyDefined(e) => format!("{}", e),
            SemanticError::InvalidReassigmentTarget(e) => format!("{}", e),
            SemanticError::InvalidReassignmentType(e) => format!("{}", e),
            SemanticError::ListInvalidReassignmentType(e) => format!("{}", e),
            SemanticError::InvalidReassigmentExpression(e) => format!("{}", e),
            SemanticError::InheritanceInvalidParent(e) => format!("{}", e),
            SemanticError::InheritanceCycle(e) => format!("{}", e),
            SemanticError::NonIterableType(e) => format!("{}", e),
            SemanticError::InvalidIndexing(e) => format!("{}", e),
            SemanticError::AccessingPrivateMember(e) => format!("{}", e),
            SemanticError::FieldNotFound(e) => format!("{}", e),
            SemanticError::MethodNotFound(e) => format!("{}", e),
            SemanticError::FieldOverride(e) => format!("{}", e),
            SemanticError::InvalidMethodOverride(e) => format!("{}", e),
        };
        write!(f, "Semantic Error: {}", suffix)
    }
}

impl From<SemanticError> for HulkError {
    fn from(e: SemanticError) -> Self {
        HulkError::SemanticError(e)
    }
}

impl HulkErrorTrait for SemanticError {
    fn get_position(&self) -> usize {
        match self {
            SemanticError::BinOpInvalidOperands(e) => e.get_position(),
            SemanticError::UnOpInvalidOperands(e) => e.get_position(),
            SemanticError::FuncAlreadyDefined(e) => e.get_position(),
            SemanticError::FuncParamsInvalidAmount(e) => e.get_position(),
            SemanticError::FuncParamInvalidType(e) => e.get_position(),
            SemanticError::FuncReturnTypeInvalid(e) => e.get_position(),
            SemanticError::TypeParamsInvalidAmount(e) => e.get_position(),
            SemanticError::TypeParamInvalidType(e) => e.get_position(),
            SemanticError::UndefinedVariable(e) => e.get_position(),
            SemanticError::UndefinedFunction(e) => e.get_position(),
            SemanticError::UndefinedType(e) => e.get_position(),
            SemanticError::UndefinedTypeOrProtocol(e) => e.get_position(),
            SemanticError::TypeOrProtocolAlreadyDefined(e) => e.get_position(),
            SemanticError::TypeMemberAlreadyDefined(e) => e.get_position(),
            SemanticError::VarDefinitionTypeMismatch(e) => e.get_position(),
            SemanticError::VarAlreadyDefined(e) => e.get_position(),
            SemanticError::InvalidReassigmentTarget(e) => e.get_position(),
            SemanticError::InvalidReassignmentType(e) => e.get_position(),
            SemanticError::ListInvalidReassignmentType(e) => e.get_position(),
            SemanticError::InvalidReassigmentExpression(e) => e.get_position(),
            SemanticError::InheritanceInvalidParent(e) => e.get_position(),
            SemanticError::InheritanceCycle(e) => e.get_position(),
            SemanticError::NonIterableType(e) => e.get_position(),
            SemanticError::InvalidIndexing(e) => e.get_position(),
            SemanticError::AccessingPrivateMember(e) => e.get_position(),
            SemanticError::FieldNotFound(e) => e.get_position(),
            SemanticError::MethodNotFound(e) => e.get_position(),
            SemanticError::FieldOverride(e) => e.get_position(),
            SemanticError::InvalidMethodOverride(e) => e.get_position(),
        }
    }
}
