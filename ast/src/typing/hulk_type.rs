use crate::TypeName;
use std::fmt::{Display, Formatter, Result};

pub type TypeAnnotation = Option<Type>;

pub fn to_string(ty: &TypeAnnotation) -> String {
    match ty {
        None => "none".to_string(),
        Some(ty) => ty.to_string(),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    BuiltIn(BuiltInType),
    Functor(FunctorType),
    Defined(TypeName),
    Iterable(Box<Type>),
}

impl From<TypeName> for Type {
    fn from(v: TypeName) -> Self {
        Self::Defined(v)
    }
}

impl From<BuiltInType> for Type {
    fn from(v: BuiltInType) -> Self {
        Self::BuiltIn(v)
    }
}

impl Type {
    pub fn new_builtin(ty: BuiltInType) -> Self {
        Type::BuiltIn(ty)
    }

    pub fn as_builtin(&self) -> Option<&BuiltInType> {
        if let Type::BuiltIn(ty) = self {
            Some(ty)
        } else {
            None
        }
    }

    pub fn as_defined(&self) -> Option<&TypeName> {
        if let Self::Defined(ty) = self {
            Some(ty)
        } else {
            None
        }
    }

    pub fn as_functor(&self) -> Option<&FunctorType> {
        if let Self::Functor(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_iterable(&self) -> Option<&Box<Type>> {
        if let Self::Iterable(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Type::BuiltIn(ty) => write!(f, "{}", ty),
            Type::Defined(ty) => write!(f, "{}", ty.id),
            Type::Functor(ty) => write!(f, "{}", ty),
            Type::Iterable(ty) => write!(f, "{}*", ty),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInType {
    Number,
    String,
    Bool,
    Object,
}

impl Display for BuiltInType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BuiltInType::Number => write!(f, "Number"),
            BuiltInType::String => write!(f, "String"),
            BuiltInType::Bool => write!(f, "Boolean"),
            BuiltInType::Object => write!(f, "Object"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctorType {
    pub parameter_types: Vec<Type>,
    pub return_type: Box<Type>,
}

impl FunctorType {
    pub fn new(parameter_types: Vec<Type>, return_type: Type) -> Self {
        Self {
            parameter_types,
            return_type: Box::new(return_type),
        }
    }
}

impl Display for FunctorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let params = self
            .parameter_types
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join("->");
        write!(f, "({}): {}", params, self.return_type)
    }
}
