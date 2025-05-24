use ast::{Identifier, TokenPosition, typing::TypeAnnotation};

#[derive(Clone)]
pub struct VarInfo {
    pub name: String,
    pub is_defined: bool,
    pub position: TokenPosition,
    pub ty: TypeAnnotation,
}

impl VarInfo {
    pub fn new(
        name: String,
        is_defined: bool,
        position: TokenPosition,
        ty: TypeAnnotation,
    ) -> Self {
        VarInfo {
            name,
            is_defined,
            position,
            ty,
        }
    }

    pub fn new_from_identifier(
        identifier: &Identifier,
        is_defined: bool,
        fallback_ty: TypeAnnotation,
    ) -> Self {
        VarInfo {
            name: identifier.id.clone(),
            is_defined,
            position: identifier.position.clone(),
            ty: if identifier.info.ty.is_some() {
                identifier.info.ty.clone()
            } else {
                fallback_ty
            },
        }
    }
}
