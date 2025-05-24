use std::collections::HashMap;

use ast::{
    BinaryOperator, UnaryOperator,
    typing::{Type, TypeAnnotation, to_string},
};

use super::{get_binary_op_functor_type, get_unary_op_functor_type};
use crate::{
    def_info::{DefinedTypeInfo, FuncInfo},
    graph_utils::{lca::LCA, parent_map_to_adj},
};

/// # Description
/// TypeChecker is responsible for checking type annotations and ensuring that
/// the types conform to the expected types in the type hierarchy.
/// It uses a Lowest Common Ancestor (LCA) algorithm to determine
/// the relationships between types in the hierarchy.
/// It also provides methods to check type conformance for binary and unary operations,
/// as well as function calls and type constructors.
/// # Fields
/// - `type_ids`: A mapping from type names to their unique ids in the type hierarchy.
/// - `type_names`: A vector of type names in the order of their ids.
/// - `type_definitions`: A mapping from type names to their TypeAnnotation.
/// - `lca`: An instance of the LCA algorithm to find the lowest common ancestor of two types in the hierarchy.
pub struct TypeChecker {
    type_ids: HashMap<String, usize>,
    type_names: Vec<String>,
    type_definitions: HashMap<String, TypeAnnotation>,
    lca: LCA,
}

impl TypeChecker {
    pub fn new(
        type_hierarchy: &HashMap<String, TypeAnnotation>,
        type_definitions: HashMap<String, TypeAnnotation>,
    ) -> Self {
        let mut type_ids = HashMap::new();
        let mut type_names = Vec::new();
        for (i, type_name) in type_hierarchy.keys().enumerate() {
            type_ids.insert(type_name.clone(), i);
            type_names.push(type_name.clone());
        }
        let adj = parent_map_to_adj(&type_hierarchy, &type_ids);
        let object_name = Type::BuiltIn(ast::typing::BuiltInType::Object).to_string();
        let root = type_ids[&object_name];
        let lca = LCA::new(&adj, root);
        TypeChecker {
            type_ids,
            type_names,
            type_definitions,
            lca,
        }
    }

    /// # Description
    /// Converts a type to its id in the type tree graph
    /// Note: it asumes that ty is defined, will panic if it is not
    fn type_to_id(&self, ty: &Type) -> usize {
        let type_name = ty.to_string();
        let id = self.type_ids.get(&type_name);
        *id.unwrap()
    }

    /// # Description
    /// Checks if two type annotations conform to each other.
    /// The rules of the conforming relationship between types are:
    /// * Every type conforms to `Object`.
    /// * Every type conforms to itself.
    /// * If `T1` inherits `T2` then `T1` conforms to `T2`.
    /// * If `T1` conforms to `T2` and `T2` conforms to `T3` then `T1` conforms to `T3`.
    /// * The only types that conform to `Number`, `String`, and `Boolean`, are respectively those same types.
    ///
    /// # Parameters
    /// - `a`: The first type annotation to check.
    /// - `b`: The second type annotation to check.
    /// # Returns
    /// `true` if the types `a` conforms to `b` each other, `false` otherwise.
    /// # Notes
    /// This function assumes that the type annotations are defined in the type hierarchy.
    pub fn conforms(&self, a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
        match (a, b) {
            (None, _) => return true,
            (_, None) => return true,
            (Some(a), Some(b)) => {
                let a_id = self.type_to_id(a);
                let b_id = self.type_to_id(b);
                if a_id == b_id {
                    return true;
                }
                let common = self.lca.get_lca(a_id, b_id);
                common == b_id
            }
        }
    }

    /// # Description
    /// Returns the lowest common supertype of two type annotations using LCA with sparse table algorithm
    /// # Parameters
    /// - `a`: The first type annotation.
    /// - `b`: The second type annotation.
    /// # Returns
    /// The lowest common supertype of the two type annotations.
    /// # Notes
    /// This function assumes that the type annotations are defined in the type hierarchy.
    pub fn get_common_supertype(&self, a: &TypeAnnotation, b: &TypeAnnotation) -> TypeAnnotation {
        match (a, b) {
            (None, _) => return b.clone(),
            (_, None) => return a.clone(),
            (Some(a), Some(b)) => {
                let a_id = self.type_to_id(a);
                let b_id = self.type_to_id(b);
                let common = self.lca.get_lca(a_id, b_id);
                let common_name = self.type_names.get(common);
                if let Some(common_name) = common_name {
                    if let Some(common_type) = self.type_definitions.get(common_name) {
                        return common_type.clone();
                    }
                }
                None
            }
        }
    }

    /// # Description
    /// Checks if operands of a binary operation conform to the expected types
    /// and returns the resulting type of the operation.
    /// # Parameters
    /// - `op`: The binary operator to check.
    /// - `left`: The left operand type annotation.
    /// - `right`: The right operand type annotation.
    /// - `errors`: A mutable vector to collect error messages if the types do not conform.
    /// # Returns
    /// The resulting type annotation of the binary operation.
    /// # Notes
    /// This function assumes that the type annotations is defined.
    pub fn check_bin_op(
        &self,
        op: &BinaryOperator,
        left: &TypeAnnotation,
        right: &TypeAnnotation,
        errors: &mut Vec<String>,
    ) -> TypeAnnotation {
        let functor = get_binary_op_functor_type(&op);

        if !self.conforms(&left, &functor.parameter_types[0])
            || !self.conforms(&right, &functor.parameter_types[1])
        {
            errors.push(format!(
                "Type mismatch: Cannot apply {} to operands of type {} and {}",
                op,
                to_string(&left),
                to_string(&right)
            ));
        }
        *functor.return_type.clone()
    }

    /// # Description
    /// Checks if operand of a unary operation conform to the expected types
    /// and returns the resulting type of the operation.
    /// # Parameters
    /// - `op`: The unary operator to check.
    /// - `operand`: The operand type annotation.
    /// - `errors`: A mutable vector to collect error messages if the types do not conform.
    /// # Returns
    /// The resulting type annotation of the unary operation.
    /// # Notes
    /// This function assumes that the type annotation is defined.
    pub fn check_up_op(
        &self,
        op: &UnaryOperator,
        operand: &TypeAnnotation,
        errors: &mut Vec<String>,
    ) -> TypeAnnotation {
        let functor = get_unary_op_functor_type(&op);

        if !self.conforms(&operand, &functor.parameter_types[0]) {
            errors.push(format!(
                "Type mismatch: Cannot apply {} to operand of type {}",
                op,
                to_string(&operand)
            ));
        }
        *functor.return_type.clone()
    }

    /// # Description
    /// Checks if a function call conforms to the expected types
    /// and returns an error if the types do not conform.
    /// # Parameters
    /// - `fn_info`: The function information containing the expected types.
    /// - `parameters`: The parameters passed to the function call.
    /// # Returns
    /// `Ok(())` if the function call conforms to the expected types,
    /// `Err(Vec<String>)` containing error messages if the types do not conform.
    /// # Notes
    /// This function assumes that the function information is defined
    /// and the parameters are valid type annotations.
    pub fn check_functor_call(
        &self,
        fn_info: &FuncInfo,
        parameters: &Vec<TypeAnnotation>,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let functor = &fn_info.functor_type;
        if functor.parameter_types.len() != parameters.len() {
            errors.push(format!(
                "Function {} expects {} parameters, but {} were provided",
                fn_info.name,
                functor.parameter_types.len(),
                parameters.len()
            ));
            return Err(errors);
        }
        for (i, (expected, provided)) in functor
            .parameter_types
            .iter()
            .zip(parameters.iter())
            .enumerate()
        {
            if !self.conforms(expected, provided) {
                errors.push(format!(
                    "Function {} expects parameter {} of type {}, but got {}",
                    fn_info.name,
                    i,
                    to_string(expected),
                    to_string(provided)
                ));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// # Description
    /// Checks if a type constructor call conforms to the expected types
    /// and returns an error if the types do not conform.
    /// # Parameters
    /// - `type_definition`: The type definition containing the expected types.
    /// - `parameters`: The parameters passed to the type constructor call.
    /// # Returns
    /// `Ok(())` if the type constructor call conforms to the expected types,
    /// `Err(Vec<String>)` containing error messages if the types do not conform.
    /// # Notes
    /// This function assumes that the type definition is defined
    /// and the parameters are valid type annotations.
    pub fn check_type_constructor(
        &self,
        type_definition: &DefinedTypeInfo,
        parameters: &Vec<TypeAnnotation>,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if type_definition.arguments_types.len() != parameters.len() {
            errors.push(format!(
                "Type {} has {} parameters, but {} were provided",
                type_definition.name.id,
                type_definition.arguments_types.len(),
                parameters.len()
            ));
            return Err(errors);
        }
        for (i, (expected, provided)) in type_definition
            .arguments_types
            .iter()
            .zip(parameters.iter())
            .enumerate()
        {
            if !self.conforms(expected, provided) {
                errors.push(format!(
                    "Type {} expects parameter {} of type {}, but got {}",
                    type_definition.name.id,
                    i,
                    to_string(expected),
                    to_string(provided)
                ));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
