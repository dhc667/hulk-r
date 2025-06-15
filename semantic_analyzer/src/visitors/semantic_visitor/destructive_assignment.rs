use ast::{
    Identifier,
    typing::{TypeAnnotation, to_string},
};
use error_handler::error::semantic::{
    definition::UndefinedVariable,
    destructive_assignment::{
        InvalidAssigmentTarget, InvalidReassignmentType, ListInvalidReassignmentType,
    },
};

use super::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    /// # Description
    /// Handles the semantic analysis for variable reassignment.
    /// It checks if the variable is defined, if it is a constant, and if the types conform.
    /// It also collects any semantic errors encountered during the analysis.
    /// # Arguments
    /// - `variable`: A reference to the `Identifier` representing the variable being reassigned.
    /// - `assignee_type`: A reference to the `TypeAnnotation` representing the type of the value being assigned.
    /// - `expr_type`: A reference to the `TypeAnnotation` representing the type of the expression being assigned.
    /// # Returns
    /// A `TypeAnnotation` representing the type of the assignment if valid, or the type of the expression if there is a semantic error.
    pub(crate) fn handle_reassign_var(
        &mut self,
        variable: &Identifier,
        assignee_type: &TypeAnnotation,
        expr_type: &TypeAnnotation,
    ) -> TypeAnnotation {
        let variable_id = variable.id.clone();
        let def_value = self.var_definitions.get_value(&variable_id);
        match def_value {
            None => {
                let error = UndefinedVariable::new(variable_id.clone(), variable.position.start);
                self.errors.push(error.into());
                expr_type.clone()
            }
            Some(def) if def.is_constant => {
                let error = InvalidAssigmentTarget::new(variable_id, variable.position.start);

                self.errors.push(error.into());
                assignee_type.clone()
            }
            Some(def) if !self.type_checker.conforms(&expr_type, &def.ty) => {
                let error = InvalidReassignmentType::new(
                    variable_id,
                    to_string(&def.ty),
                    to_string(&expr_type),
                    variable.position.start,
                );
                self.errors.push(error.into());
                assignee_type.clone()
            }
            Some(_) => assignee_type.clone(),
        }
    }

    /// # Description
    /// Handles the semantic analysis for field reassignment.
    /// It checks if the field is defined, if it is a constant, and if the types conform.
    /// It also collects any semantic errors encountered during the analysis.
    /// # Arguments
    /// - `field`: A reference to the `Identifier` representing the field being reassigned.
    /// - `assignee_type`: A reference to the `TypeAnnotation` representing the type of the value being assigned.
    /// - `expr_type`: A reference to the `TypeAnnotation` representing the type of the expression being assigned.
    /// # Returns
    /// A `TypeAnnotation` representing the type of the assignment if valid, or the type of the expression if there is a semantic error.
    /// # Notes
    /// This function does not check if the field is accessible, assumes that that has been done previously.
    pub(crate) fn handle_field_reassign(
        &mut self,
        field: &Identifier,
        assignee_type: &TypeAnnotation,
        expr_type: &TypeAnnotation,
    ) -> TypeAnnotation {
        let member_name = field.id.clone();
        if !self.type_checker.conforms(&expr_type, &assignee_type) {
            let error = InvalidReassignmentType::new(
                member_name,
                to_string(&assignee_type),
                to_string(&expr_type),
                field.position.start,
            );
            self.errors.push(error.into());
        }
        assignee_type.clone()
    }

    /// # Description
    /// Handles the semantic analysis for list element reassignment.
    /// It checks if the types conform for the reassignment of a list element.
    /// It also collects any semantic errors encountered during the analysis.
    /// # Arguments
    /// - `assignee_type`: A reference to the `TypeAnnotation` representing the type of the list element being assigned.
    /// - `expr_type`: A reference to the `TypeAnnotation` representing the type of the expression being assigned to the list element.
    /// # Returns
    /// A `TypeAnnotation` representing the type of the assignment if valid, or the type of the expression if there is a semantic error.
    pub(crate) fn handle_list_element_reassign(
        &mut self,
        assignee_type: &TypeAnnotation,
        expr_type: &TypeAnnotation,
        position: usize,
    ) -> TypeAnnotation {
        if !self.type_checker.conforms(&expr_type, &assignee_type) {
            let error = ListInvalidReassignmentType::new(
                to_string(&expr_type),
                to_string(&assignee_type),
                position,
            );
            self.errors.push(error.into());
        }
        assignee_type.clone()
    }
}
