use ast::{FunctionDef, Identifier, typing::FunctorType};

/// # Description
/// `FuncInfo` is a struct that encapsulates information about a function definition.
/// It includes the function's name, its parameters, and its type information.
/// It is used in the semantic analyzer to store and retrieve function definitions
/// and their associated metadata.
///
/// # Parameters
/// - `name`: The identifier representing the function's name.
/// - `parameters`: A vector of identifiers representing the function's parameters.
/// - `functor_type`: The type information of the function, including parameter and return types.
///
/// # Methods
/// - `new(name, parameters)`: Constructs a new `FuncInfo` from a function name and its parameters.
/// - `get_type_wrapper_name(info)`: Returns a string representing the type wrapper name for the function.
/// - `get_var_instance_name(info)`: Returns a string representing the variable instance name for the function.
///
/// # Conversion
/// Implements `From<&FunctionDef>` for `FuncInfo`, allowing conversion from a function definition AST node.
///
#[derive(Clone)]
pub struct FuncInfo {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub functor_type: FunctorType,
}

impl FuncInfo {
    pub fn new(name: Identifier, parameters: Vec<Identifier>) -> Self {
        let functor_type = FunctorType::new(
            parameters.iter().map(|id| id.info.ty.clone()).collect(),
            None,
        );

        Self {
            name,
            parameters,
            functor_type,
        }
    }

    /// Returns the type wrapper name for the function.
    /// Example: If the function name is `foo`, the wrapper name will be `$fooTypeWrapper`.
    pub fn get_type_wrapper_name(info: &FuncInfo) -> String {
        format!("${}TypeWrapper", info.name.clone())
    }

    /// Returns the variable instance name for the function.
    /// Example: If the function name is `foo`, the instance name will be `$fooInstance`.
    pub fn get_var_instance_name(info: &FuncInfo) -> String {
        format!("${}Instance", info.name.clone())
    }
}

impl From<&FunctionDef> for FuncInfo {
    fn from(func_def: &FunctionDef) -> Self {
        let parameters = func_def.parameters.clone();
        Self {
            name: func_def.identifier.clone(),
            parameters,
            functor_type: FunctorType::new(
                func_def
                    .parameters
                    .iter()
                    .map(|id| id.info.ty.clone())
                    .collect(),
                func_def.identifier.info.ty.clone(),
            ),
        }
    }
}
