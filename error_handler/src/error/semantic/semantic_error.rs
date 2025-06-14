pub enum SemanticError {
    // Type errors

    // operator
    BinOpInvalidOperands, // "Type mismatch: Cannot apply {} to operands of type {} and {}",
    UnOpInvalidOperands,  //"Type mismatch: Cannot apply {} to operand of type {}",

    // function
    FuncAlreadyDefined,      // "Function {} is already defined",
    FuncParamsInvalidAmount, // "Function {} expects {} parameters, but {} were provided",
    FuncParamInvalidType,    // "Function {} expects parameter {} of type {}, but got {}",
    FuncReturnTypeInvalid,   // "Type mismatch: Function {} returns {} but {} was found",

    // type constructor
    TypeParamsInvalidAmount, // "Type {} has {} parameters, but {} were provided",
    TypeParamInvalidType,    // "Type {} expects parameter {} of type {}, but got {}"

    // definition
    UndefinedVariable,       // "Variable {} is not defined"
    UndefinedFunction,       // "Function {} is not defined",
    UndefinedType,           // "Type {} is not defined",
    UndefinedTypeOrProtocol, // "Semantic Error: Type or protocol {} is not defined.",

    // type definition
    TypeOrProtocolAlreadyDefined, // "Already exists a type or protocol {}"
    TypeMemberAlreadyDefined,     // "Member {} is already defined in type {}",

    // variable definition
    VarDefinitionTypeMismatch, // "Type mismatch: Cannot assign {} to {}",
    VarAlreadyDefined,         // "Constant {} is already defined"

    // destructive assignment
    InvalidReassigmentTarget, // "Semantic Error: `{}` is not a valid assignment target",
    InvalidReassignmentType,  // "Type mismatch: {} is {} but is being reassigned with {}",
    ListInvalidReassignmentType, // "Type mismatch: Cannot assign {} to list element of type {}",
    InvalidReassigmentExpression, // "Semantic Error: only variables and self properties can be assigned"

    // inheritance
    InheritanceInvalidParent, // "Type {} is a built-in type and cannot be inherited from",
    InheritanceCycle,         // "Semantic Error: Inheritance cycle detected

    // iterable
    NonIterableType, // "Type mismatch: Cannot iterate over type {}",
    InvalidIndexing, // "Type mismatch: Cannot use index of type {} to access iterable",

    // member access
    AccessingPrivateMember, // "Cannot access member {} of type {}. Properties are private, even to inherited types.",
    FieldNotFound,          // "Could not find data member {}"
    MethodNotFound,         // "Could not find method {},

    // override
    FieldOverride, // "Semantic Error: Cannot declare field {} in type {}, as it overrides parent definition.",
    InvalidMethodOverride, // "Semantic Error: Method {} in type {}, does not properly overrides parent definition.",
}
