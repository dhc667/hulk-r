use ast::TokenPosition;

use crate::Type;

pub struct VariableInfo {
    pub name: String,
    pub is_defined: bool,
    pub definition_pos: TokenPosition,
    pub ty: Option<Type>,
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
