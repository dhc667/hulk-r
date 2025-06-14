use crate::{FunctionDef, Identifier, Keyword, TypeName};

use super::{DataMemberDef, InheritanceIndicator};

#[derive(Debug)]
pub struct TypeDef {
    pub type_token: Keyword,
    pub name: TypeName,
    pub parameter_list: Vec<Identifier>,
    pub inheritance_indicator: Option<InheritanceIndicator>,
    pub data_member_defs: Vec<DataMemberDef>,
    pub function_member_defs: Vec<FunctionDef>,
}

impl TypeDef {
    pub fn new(
        type_token: Keyword,
        name: TypeName,
        parameter_list: Vec<Identifier>,
        inheritance_indicator: Option<InheritanceIndicator>,
        data_member_defs: Vec<DataMemberDef>,
        function_member_defs: Vec<FunctionDef>,
    ) -> Self {
        Self {
            type_token,
            name,
            parameter_list,
            inheritance_indicator,
            data_member_defs,
            function_member_defs,
        }
    }
}
