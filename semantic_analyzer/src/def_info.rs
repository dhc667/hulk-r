use ast::{TokenPosition, typing::TypeAnnotation};

pub struct DefinitionInfo {
    pub name: String,
    pub is_defined: bool,
    pub position: TokenPosition,
    pub ty: TypeAnnotation,
}
