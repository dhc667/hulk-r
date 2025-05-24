use ast::{Identifier, TokenPosition, typing::TypeAnnotation};

/// # Description
/// `VarInfo` is a struct that encapsulates information about a variable definition.
/// It includes the variable's name, whether it is defined, its position in the source code,
/// and its type annotation.
/// # Fields
/// - `name`: The name of the variable.
/// - `is_defined`: A boolean indicating whether the variable is defined.
/// - `position`: The position of the variable in the source code, represented by a `TokenPosition`.
/// - `ty`: The type annotation of the variable, represented by a `TypeAnnotation`.
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
