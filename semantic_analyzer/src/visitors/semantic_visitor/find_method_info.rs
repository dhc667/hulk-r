use ast::typing::TypeAnnotation;

use crate::def_info::FuncInfo;

use super::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    /// Searches for a medthod's definition information within a type definition or its parent types.
    ///
    /// Traverses the type hierarchy, looking for the specified method name.
    ///
    /// # Parameters
    /// - `member_name`: The name of the member to search for.
    /// - `ty`: The type annotation to start the search from.
    ///
    /// # Returns
    /// An `Option<&FuncInfo>`: A reference to the `FuncInfo` if the method is found, or `None` if it is not found.
    pub(crate) fn find_method_info(
        &self,
        member_name: String,
        ty: &TypeAnnotation,
    ) -> Option<&FuncInfo> {
        let mut current_type = ty.clone();
        loop {
            let Some(ty) = &current_type else { break };

            let type_name = ty.to_string();
            let type_def = self.type_definitions.get_value(&type_name);

            let Some(type_def) = type_def.and_then(|d| d.as_defined()) else {
                current_type = None;
                continue;
            };
            if let Some(info) = type_def.members.get(&member_name).and_then(|d| d.as_func()) {
                return Some(info);
            }
            // Try parent type
            let parent_type = self.type_hierarchy.get(&type_name).cloned().expect(&format!(
                "Type name {} is not found in type tree, this should not happen in semantic visitor",
                type_name
            ));
            current_type = parent_type;
        }
        None
    }
}
