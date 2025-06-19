use ast::typing::{Type, TypeAnnotation, to_string};
use error_handler::error::{
    error::HulkError,
    semantic::{definition::UndefinedTypeOrProtocol, inheritance::ObjectAnnotationError},
};

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

        if to_string(&Some(ty.clone())) == "Object" {
            return Err(ObjectAnnotationError::new(position).into());
        }
        //TODO: we probably need to do something generic for this
        if let Type::Iterable(inner) = ty {
            return self.get_conformable(&Some(inner.as_ref().clone()), position);
        };

        if self.is_type_defined(&ty) {
            return Ok(annotation.clone());
        }
        Err(UndefinedTypeOrProtocol::new(ty.to_string(), position).into())
    }
}
