use ast::{
    Expression, VisitableExpression,
    typing::{BuiltInType, Type, TypeAnnotation, to_string},
};

use super::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    /// # Description
    /// Handles the semantic analysis for the `print` function.
    /// It checks if the number of arguments is correct and if the argument type is valid.
    /// # Arguments
    /// - `arguments`: A mutable reference to a vector of `Expression` representing the arguments passed to the `print` function.
    /// # Returns
    /// A `TypeAnnotation` representing the type of the argument if it is valid, or `None` if there is a type mismatch.
    pub(crate) fn handle_print(&mut self, arguments: &mut Vec<Expression>) -> TypeAnnotation {
        if arguments.len() != 1 {
            let message = format!(
                "Type mismatch: print function expects 1 argument, but {} were provided",
                arguments.len()
            );
            self.errors.push(message);
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
            let message = format!(
                "Type mismatch: print function expects argument of type String, but {} was provided",
                to_string(&arg_type)
            );
            self.errors.push(message);
            return None;
        }
        return arg_type;
    }
}
