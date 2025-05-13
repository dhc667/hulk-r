use std::{fmt::Display, usize};

use crate::typing::{Type, TypeAnnotation};

use super::*;

pub struct Identifier {
    pub position: TokenPosition,
    pub id: String,
    pub info: IdentifierInfo,
}

impl Identifier {
    pub fn new(start: usize, end: usize, id: &str) -> Self {
        Identifier {
            position: TokenPosition::new(start, end),
            id: id.to_string(),
            info: IdentifierInfo::new(),
        }
    }

    pub fn annotate_type(&mut self, ty: Type) {
        self.info.ty = Some(ty);
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub struct IdentifierInfo {
    pub ty: TypeAnnotation,
    pub definition_pos: Option<TokenPosition>,
}

impl IdentifierInfo {
    pub fn new() -> Self {
        IdentifierInfo {
            ty: None,
            definition_pos: None,
        }
    }
}
