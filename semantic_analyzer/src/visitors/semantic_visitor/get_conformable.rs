use ast::typing::{Type, TypeAnnotation};

use super::SemanticVisitor;

impl<'a> SemanticVisitor<'a> {
    pub(crate) fn get_conformable(
        &self,
        annotation: &TypeAnnotation,
    ) -> Result<TypeAnnotation, String> {
        let ty = if let Some(annotation) = annotation.as_ref() {
            annotation
        } else {
            return Ok(None);
        };

        //TODO: we probably need to do something generic for this
        let ty = if let Type::Iterable(inner) = ty {
            inner.as_ref()
        } else {
            ty
        };

        if self.type_definitions.is_defined(&ty.to_string()) {
            return Ok(annotation.clone());
        }
        Err(format!(
            "Semantic Error: Type or protocol {} is not defined.",
            ty.to_string()
        ))
    }
}
