use crate::{Keyword, TypeName};

use super::{ExtensionIndicator, FunctionSignature};

pub struct ProtocolDef {
    pub protocol_token: Keyword,
    pub name: TypeName,
    pub extension_indicator: Option<ExtensionIndicator>,
    pub function_signatures: Vec<FunctionSignature>,
}

impl ProtocolDef {
    pub fn new(
        protocol_token: Keyword,
        name: TypeName,
        extension_indicator: Option<ExtensionIndicator>,
        function_signatures: Vec<FunctionSignature>,
    ) -> Self {
        Self {
            protocol_token,
            name,
            extension_indicator,
            function_signatures,
        }
    }
}
