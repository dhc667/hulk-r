use crate::Identifier;

pub struct FunctionSignature {
    pub identifier: Identifier,
    pub parameters: Vec<Identifier>,
}

impl FunctionSignature {
    pub fn new(identifier: Identifier, parameters: Vec<Identifier>) -> Self {
        Self {
            identifier,
            parameters,
        }
    }
}
