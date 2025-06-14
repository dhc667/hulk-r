#[derive(Debug)]
pub enum ReturnType {
    ExpressionList(Vec<i32>),
    Expression(i32),
    Token,
}

impl ReturnType {
    pub fn as_expression_list(self) -> Option<Vec<i32>> {
        if let Self::ExpressionList(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_expression(self) -> Option<i32> {
        if let Self::Expression(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
