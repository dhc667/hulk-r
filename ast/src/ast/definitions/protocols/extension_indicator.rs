use crate::{Keyword, TypeName};


pub struct ExtensionIndicator {
    pub extends_token: Keyword,
    pub extendee_name: TypeName,
}

impl ExtensionIndicator {
    pub fn new(extends_token: Keyword, extendee_name: TypeName) -> Self {
        Self {
            extends_token,
            extendee_name,
        }
    }
}
