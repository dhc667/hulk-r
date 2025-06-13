use crate::{Expression, Keyword, TypeName};

pub struct InheritanceIndicator {
    pub inherits_token: Keyword,
    pub parent_name: TypeName,
    pub argument_list: Vec<Expression>,
}

impl InheritanceIndicator {
    pub fn new(
        inherits_token: Keyword,
        parent_name: TypeName,
        argument_list: Vec<Expression>,
    ) -> Self {
        Self {
            inherits_token,
            parent_name,
            argument_list,
        }
    }
}
