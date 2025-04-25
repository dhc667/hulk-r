use std::fmt::Display;

use crate::{Identifier, Visitable, Visitor, typing::TypeAnotation};

pub struct Variable {
    pub identifier: Identifier,
    pub typing: TypeAnotation,
}

impl Variable {
    pub fn new(identifier: Identifier, typing: TypeAnotation) -> Self {
        Variable { identifier, typing }
    }
}

impl<T: Visitor<R>, R> Visitable<T, R> for Variable {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_variable(self)
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
