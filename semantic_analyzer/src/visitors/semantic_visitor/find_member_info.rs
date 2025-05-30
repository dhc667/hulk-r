use ast::typing::TypeAnnotation;

use crate::def_info::DefinitionInfo;

use super::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    /// # Description
	/// `find_member_info` searches for a member's definition information
	/// within a type definition or its parent types.
	/// It traverses the type hierarchy, looking for the specified member name.
	/// # Parameters
	/// - `member_name`: The name of the member to search for.
	/// - `ty`: The type annotation to start the search from.
	/// - `with_lookup`: A boolean indicating whether to look up in the inheritance tree.
	/// # Returns
	/// - `Option<&DefinitionInfo>`: Returns an `Option` containing a reference to the `DefinitionInfo`
	/// if the member is found, or `None` if it is not found.
    pub(crate) fn find_member_info(
        &self,
        member_name: String,
        ty: &TypeAnnotation,
        with_lookup: bool,
    ) -> Option<&DefinitionInfo> {
        let mut current_type = ty.clone();
        loop {
            let Some(ty) = &current_type else { break };

            let type_name = ty.to_string();
            let type_def = self.type_definitions.get_value(&type_name);

            if let Some(type_def) = type_def.and_then(|d| d.as_defined()) {
                if let Some(info) = type_def.members.get(&member_name) {
                    return Some(info);
                }

                // If we are not looking up in the inheritance tree, we can stop here
                if !with_lookup {
                    return None;
                }

                // Try parent type
                let parent_type = self.type_hierarchy.get(&type_name).cloned().expect(&format!(
                   "Type name {} is not found in type tree, this should not happen in semantic visitor",
                    type_name 
                ));
                current_type = parent_type;
                continue;
            }
            current_type = None;
        }
        None
    }
    
}