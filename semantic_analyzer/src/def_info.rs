use ast::{Identifier, TokenPosition, typing::TypeAnnotation};

#[derive(Clone)]
pub struct DefinitionInfo {
    pub name: String,
    pub is_defined: bool,
    pub position: TokenPosition,
    pub ty: TypeAnnotation,
}

impl DefinitionInfo {
    pub fn new(
        name: String,
        is_defined: bool,
        position: TokenPosition,
        ty: TypeAnnotation,
    ) -> Self {
        DefinitionInfo {
            name,
            is_defined,
            position,
            ty,
        }
    }

    pub fn new_from_identifier(
        identifier: &Identifier,
        is_defined: bool,
        ty: TypeAnnotation,
    ) -> Self {
        DefinitionInfo {
            name: identifier.id.clone(),
            is_defined,
            position: identifier.position.clone(),
            ty: if identifier.info.ty.is_some() {
                identifier.info.ty.clone()
            } else {
                ty
            },
        }
    }
}
