use ast::{
    FunctionDef, TypeName, VisitableExpression,
    typing::{TypeAnnotation, to_string},
};
use error_handler::error::semantic::function::FuncReturnTypeInvalid;

use crate::def_info::VarInfo;

use super::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    /// # Description
    /// Handles the semantic analysis for function definitions.
    /// Defines the function parameters in the variable definitions,
    /// checks the body of the function, and verifies that the return type matches the expected type.
    /// It also collects any semantic errors encountered during the analysis.
    /// # Arguments
    /// - `fn_def`: A mutable reference to the `FunctionDef` representing the function being defined in the AST.
    /// - `enclosing_type_name`: An optional reference to the `TypeName` of the enclosing type, if the function is a method of a type.
    /// # Returns
    /// A `TypeAnnotation` representing the return type of the function definition.
    pub(crate) fn handle_fn_def(
        &mut self,
        fn_def: &mut FunctionDef,
        enclosing_type_name: Option<&TypeName>,
    ) -> TypeAnnotation {
        for param in &fn_def.parameters {
            self.var_definitions.define(
                param.id.clone(),
                VarInfo::new_from_identifier(param, true, None),
            );
        }

        let body_type = fn_def.body.accept(self);

        let fn_info = match enclosing_type_name {
            Some(type_name) => {
                self.type_definitions
                    .get_value_mut(&type_name.id)
                    .and_then(|d| d.as_defined_mut())
                    .expect(&format!(
                        "Type {} is not defined, this should not happen in Semantic visitor",
                        &type_name.id
                    ))
                    .members
                    .get_mut(&fn_def.identifier.id)
                    .and_then(|d| d.as_func_mut())
                    .expect(&format!(
                        "Method {} is not defined in type {}, this should not happen in Semantic visitor",
                        &fn_def.identifier.id, &type_name.id
                ))
            }
            None => {
                self.func_definitions
                    .get_value_mut(&fn_def.identifier.id)
                    .expect(&format!(
                        "Function {} is not defined, this should not happen in Semantic visitor",
                        &fn_def.identifier.id,
                    ))
            }
        };

        if !self
            .type_checker
            .conforms(&body_type, &fn_info.get_functor_type().return_type)
        {
            self.errors.push(
                FuncReturnTypeInvalid::new(
                    fn_def.identifier.id.clone(),
                    to_string(&fn_info.get_functor_type().return_type),
                    to_string(&body_type),
                    fn_def.identifier.position.start,
                )
                .into(),
            );
        }
        // annotate the function identifier with return type in the AST
        fn_def.identifier.set_type_if_none(body_type.clone());
        // annotate the function identifier with return type in the info
        fn_info.name.set_type_if_none(body_type.clone());
        None
    }
}
