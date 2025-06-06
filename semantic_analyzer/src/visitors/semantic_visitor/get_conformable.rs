use ast::typing::Type;

use crate::typing::get_conformable::GetConformable;

use super::SemanticVisitor;

impl<'a> GetConformable for SemanticVisitor<'a> {
    fn is_type_defined(&self, ty: &Type) -> bool {
        self.type_definitions.is_defined(&ty.to_string())
    }
}
