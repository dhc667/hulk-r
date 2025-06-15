use ast::{
    Expression, VisitableExpression,
    typing::{BuiltInType, Type, TypeAnnotation, to_string},
};
use error_handler::error::semantic::function::{FuncParamInvalidType, FuncParamsInvalidAmount};

use super::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    /// # Description
    /// Handles the semantic analysis for the `print` function.
    /// It checks if the number of arguments is correct and if the argument type is valid.
    /// # Arguments
    /// - `arguments`: A mutable reference to a vector of `Expression` representing the arguments passed to the `print` function.
    /// # Returns
    /// A `TypeAnnotation` representing the type of the argument if it is valid, or `None` if there is a type mismatch.
    pub(crate) fn handle_print(
        &mut self,
        arguments: &mut Vec<Expression>,
        position: usize,
    ) -> TypeAnnotation {
        if arguments.len() != 1 {
            let error =
                FuncParamsInvalidAmount::new("print".to_string(), 1, arguments.len(), position);
            self.errors.push(error.into());
            return None;
        }
        let arg_type = arguments[0].accept(self);
        if !vec![
            Some(Type::BuiltIn(BuiltInType::String)),
            Some(Type::BuiltIn(BuiltInType::Number)),
            Some(Type::BuiltIn(BuiltInType::Bool)),
        ]
        .contains(&arg_type)
        {
            let error = FuncParamInvalidType::new(
                "print".to_string(),
                0,
                "Printable".to_string(),
                to_string(&arg_type),
                position,
            );
            self.errors.push(error.into());
            return None;
        }
        return arg_type;
    }
}
