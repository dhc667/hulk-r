use crate::{typing::Type, BinaryOperator, Expression, Identifier, Keyword};

pub struct ConstantDef {
    pub identifier: Identifier,
    pub assignment_operator: BinaryOperator,
    pub constant_token: Keyword,
    pub initializer_expression: Expression,
}

impl ConstantDef {
    pub fn new(
        constant_token: Keyword,
        type_annotation: Type,
        mut identifier: Identifier,
        assignment_operator: BinaryOperator,
        initializer_expression: Expression,
    ) -> Self {
        identifier.annotate_type(type_annotation);

        Self {
            identifier,
            assignment_operator,
            constant_token,
            initializer_expression,
        }
    }
}
