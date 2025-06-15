use ast::{
    Identifier,
    typing::{TypeAnnotation, to_string},
};
use error_handler::error::semantic::variable_definition::{
    VarAlreadyDefined, VarDefinitionTypeMismatch,
};

use crate::{def_info::VarInfo, visitors::SemanticVisitor};

impl<'a> SemanticVisitor<'a> {
    /// # Description
    /// Handles the semantic analysis for variable definitions.
    /// It checks if the variable is already defined, if it is shadowable,
    /// and if the type of the right-hand side conforms to the type of the variable.
    /// Defines the variable in the variable definitions if all checks pass.
    /// Annotates the identifier with the type of the variable and sets its definition position.
    /// It also collects any semantic errors encountered during the analysis.
    /// # Arguments
    /// - `identifier`: A mutable reference to the `Identifier` representing the variable being defined.
    /// - `right_type`: A `TypeAnnotation` representing the type of the value being assigned to the variable.
    /// - `shadoweable`: A boolean indicating whether the variable can be shadowed (redefined) in the current scope.
    /// # Returns
    /// A `TypeAnnotation` representing the type of the variable if it is defined successfully, or `None` if there is a semantic error.
    pub(crate) fn handle_var_definition(
        &mut self,
        identifier: &mut Identifier,
        right_type: TypeAnnotation,
        shadoweable: bool,
    ) -> TypeAnnotation {
        let is_asignable = self.type_checker.conforms(&right_type, &identifier.info.ty);

        if !is_asignable {
            let error = VarDefinitionTypeMismatch::new(
                to_string(&right_type),
                to_string(&identifier.info.ty),
                identifier.position.start,
            );
            self.errors.push(error.into());
        }

        if !shadoweable && self.var_definitions.is_defined(&identifier.id) {
            let error = VarAlreadyDefined::new(identifier.id.clone(), identifier.position.start);
            self.errors.push(error.into());
        } else {
            let var_info = if shadoweable {
                VarInfo::new_from_identifier(identifier, true, right_type.clone())
            } else {
                VarInfo::new_constant_from_identifier(identifier, true, right_type.clone())
            };
            self.var_definitions.define(identifier.id.clone(), var_info);
        }
        identifier.set_type_if_none(right_type.clone());
        identifier.info.definition_pos = Some(identifier.position.clone());
        None
    }

    pub(crate) fn handle_field_definition(
        &mut self,
        identifier: &mut Identifier,
        right_type: TypeAnnotation,
    ) -> TypeAnnotation {
        let is_asignable = self.type_checker.conforms(&right_type, &identifier.info.ty);

        if !is_asignable {
            let error = VarDefinitionTypeMismatch::new(
                to_string(&right_type),
                to_string(&identifier.info.ty),
                identifier.position.start,
            );
            self.errors.push(error.into());
        }

        identifier.set_type_if_none(right_type.clone());
        identifier.info.definition_pos = Some(identifier.position.clone());
        None
    }
}
