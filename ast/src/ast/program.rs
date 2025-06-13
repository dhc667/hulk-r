use super::{Definition, Expression};

pub struct Program {
    pub definitions: Vec<Definition>,
    pub expressions: Vec<Expression>,
}

impl Program {
    pub fn new(definitions: Vec<Definition>, expressions: Vec<Expression>) -> Self {
        Self {
            definitions,
            expressions,
        }
    }
}
