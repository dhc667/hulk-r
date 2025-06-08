use ast::typing::TypeAnnotation;

use crate::def_info::VarInfo;

use super::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    /// # Description
    /// `find_member_info` searches for a member's definition information
    /// within a type definition.
    /// # Parameters
    /// - `member_name`: The name of the member to search for.
    /// - `ty`: The type annotation to start the search from.
    /// # Returns
    /// - `Option<&VarInfo>`: Returns an `Option` containing a reference to the `VarInfo`
    /// if the member is found, or `None` if it is not found.
    pub(crate) fn find_member_info(
        &self,
        member_name: String,
        ty: &TypeAnnotation,
    ) -> Option<&VarInfo> {
        let Some(ty) = ty else {
            return None;
        };

        let type_name = ty.to_string();
        // Get the type definition by its name
        let Some(type_def) = self
            .type_definitions
            .get_value(&type_name)
            .and_then(|d| d.as_defined())
        else {
            // If the type definition is not found, return None
            return None;
        };

        type_def.members.get(&member_name).and_then(|d| d.as_var())
    }
}
