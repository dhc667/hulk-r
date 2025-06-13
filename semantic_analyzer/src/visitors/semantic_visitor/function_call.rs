use ast::{Expression, Identifier, VisitableExpression, typing::TypeAnnotation};

use crate::def_info::FuncInfo;

use super::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    /// # Description
    /// Handles the semantic analysis for function calls.
    /// It checks if the function is defined, if the arguments conform to the expected types,
    /// and sets the type of the identifier to the return type of the function. It also collects any semantic errors encountered during the analysis.
    /// # Arguments
    /// - `fn_info`: A reference to the `FuncInfo` containing information about the function being called.
    /// - `identifier`: A mutable reference to the `Identifier` representing the function call.
    /// - `arguments`: A mutable reference to a vector of `Expression` representing the arguments passed to the function.
    /// # Returns
    /// A `TypeAnnotation` representing the return type of the function call.
    pub(crate) fn handle_function_call(
        &mut self,
        fn_info: FuncInfo,
        identifier: &mut Identifier,
        arguments: &mut Vec<Expression>,
    ) -> TypeAnnotation {
        let fn_info = fn_info.clone();

        let parameter_types: Vec<TypeAnnotation> =
            arguments.iter_mut().map(|arg| arg.accept(self)).collect();
        let fn_check_result = self
            .type_checker
            .check_functor_call(&fn_info, &parameter_types);
        if let Err(errors) = fn_check_result {
            for error in errors {
                self.errors.push(error);
            }
        }
        identifier.set_type_if_none(*fn_info.get_functor_type().return_type.clone());
        *fn_info.get_functor_type().return_type.clone()
    }
}
