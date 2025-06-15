use ast::typing::{Type, TypeAnnotation};
use error_handler::error::{error::HulkError, semantic::definition::UndefinedTypeOrProtocol};

pub trait GetConformable {
    fn is_type_defined(&self, ty: &Type) -> bool;

    fn get_conformable(
        &self,
        annotation: &TypeAnnotation,
        position: usize,
    ) -> Result<TypeAnnotation, HulkError> {
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

        if self.is_type_defined(&ty) {
            return Ok(annotation.clone());
        }
        Err(UndefinedTypeOrProtocol::new(ty.to_string(), position).into())
    }
}
