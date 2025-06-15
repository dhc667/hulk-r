use crate::{Identifier, Keyword};

use super::{FunctionBody, FunctionDef, FunctionSignature};

#[derive(Debug)]
pub struct GlobalFunctionDef {
    pub function_token: Keyword,
    pub function_def: FunctionDef,
}

impl GlobalFunctionDef {
    pub fn new(
        function_token: Keyword,
        identifier: Identifier,
        parameters: Vec<Identifier>,
        body: FunctionBody,
    ) -> Self {
        GlobalFunctionDef {
            function_token,
            function_def: FunctionDef::new(identifier, parameters, body),
        }
    }

    pub fn from_signature_and_body(
        function_token: Keyword,
        signature: FunctionSignature,
        body: FunctionBody,
    ) -> Self {
        Self {
            function_token,
            function_def: FunctionDef::from_signature(signature, body),
        }
    }
}
