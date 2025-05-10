use super::TokenPosition;

/// # Description
///
/// This is used for type annotations and the name tokens of protocols and types, in
/// order to differentiate it from Identifiers, which hold type data
#[derive(Debug, Clone)]
pub struct TypeName {
    pub id: String,
    pub position: TokenPosition,
}

impl TypeName {
    pub fn new(start: usize, end: usize, id: String) -> Self {
        Self {
            id,
            position: TokenPosition::new(start, end),
        }
    }
}

impl PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
