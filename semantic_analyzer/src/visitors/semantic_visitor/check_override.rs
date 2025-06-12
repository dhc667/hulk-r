use ast::typing::Type;

use crate::visitors::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    /// # Description
    /// Checks if a method overrides another method correctly in the class hierarchy.
    /// # Arguments
    /// * `method_info` - The info of the method to check for override.
    /// * `ty` - The type of the Type where the method is defined.
    /// # Returns
    /// A boolean indicating whether the method overrides another method correctly.
    /// # Note
    /// This function assumes that the type definitions and hierarchy are already populated.
    /// And that the method is defined in the type.
    pub(crate) fn check_method_override(&self, method_name: &str, type_name: &str) -> bool {
        let type_info = self
            .type_definitions
            .get_value(type_name)
            .and_then(|d| d.as_defined())
            .expect(&format!(
                "Type {} is not defined, this should not happen in Semantic visitor, check method override",
                type_name
            ));

        let parent = self
            .type_hierarchy
            .get(&type_info.name.id)
            .cloned()
            .flatten();

        let Some(parent) = parent else {
            return true;
        };
        let Type::Defined(_) = parent else {
            return true;
        };

        let parent = Some(parent);

        let method_info = type_info
            .members
            .get(method_name)
            .and_then(|d| d.as_func())
            .expect(&format!(
                "Method {} is not defined in type {}, this should not happen in Semantic visitor, check method override",
                method_name, type_name
            ));

        let overriden_method = self.find_method_info(method_name.to_string(), &parent);
        let Some(overriden_method) = overriden_method else {
            return true;
        };

        self.type_checker
            .implements_variant(method_info, overriden_method)
    }

    pub(crate) fn check_field_override(&self, field_name: &str, type_name: &str) -> bool {
        let type_info = self
            .type_definitions
            .get_value(type_name)
            .and_then(|d| d.as_defined())
            .expect(&format!(
                "Type {} is not defined, this should not happen in Semantic visitor, check field override",
                type_name
            ));

        let parent = self
            .type_hierarchy
            .get(&type_info.name.id)
            .cloned()
            .flatten();

        let Some(parent) = parent else {
            return true;
        };
        let Type::Defined(_) = parent else {
            return true;
        };

        let parent = Some(parent);

        let overriden_field = self.find_member_info_lookup(field_name.to_string(), &parent);
        !overriden_field.is_some() // fields cannot be overridden, so if we find one, it's an error
    }
}
