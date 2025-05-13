use crate::TypeName;

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

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::BuiltIn(ty) => match ty {
                BuiltInType::Number => "Number".to_string(),
                BuiltInType::String => "String".to_string(),
                BuiltInType::Bool => "Bool".to_string(),
                BuiltInType::Object => "Object".to_string(),
            },
            Type::Defined(ty) => ty.id.to_string(),
            Type::Functor(ty) => ty.to_string(),
            Type::Iterable(ty) => format!("{}*", ty.to_string()),
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

impl ToString for FunctorType {
    fn to_string(&self) -> String {
        format!(
            "({}): {}",
            self.parameter_types
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.return_type.to_string()
        )
    }
}

