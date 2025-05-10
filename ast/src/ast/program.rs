use super::{Definition, Expression};

pub struct Program {
    pub definitions: Vec<Definition>,
    pub main_expression: Expression,
}

impl Program {
    pub fn new(definitions: Vec<Definition>, main_expression: Expression) -> Self {
        Self {
            definitions,
            main_expression,
        }
    }
}
