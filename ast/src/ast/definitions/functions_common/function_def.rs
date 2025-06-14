use crate::{FunctionSignature, Identifier};

use super::FunctionBody;

#[derive(Debug)]
pub struct FunctionDef {
    pub identifier: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: FunctionBody,
}

impl FunctionDef {
    pub fn new(identifier: Identifier, parameters: Vec<Identifier>, body: FunctionBody) -> Self {
        Self {
            identifier,
            parameters,
            body,
        }
    }

    pub fn from_signature(signature: FunctionSignature, body: FunctionBody) -> Self {
        Self {
            identifier: signature.identifier,
            parameters: signature.parameters,
            body,
        }
    }
}
