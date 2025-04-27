use ast::TokenPosition;

use crate::TypeAnnotation;

pub struct VariableInfo {
    pub name: String,
    pub is_defined: bool,
    pub definition_pos: TokenPosition,
    pub ty: TypeAnnotation,
}

impl VariableInfo {
    pub fn new(name: String, definition_pos: TokenPosition) -> Self {
        VariableInfo {
            name,
            is_defined: false,
            definition_pos,
            ty: None,
        }
    }
}
