pub type TypeAnnotation = Option<Type>;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    BuiltIn(BuiltInType), //Defined(DefinedType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInType {
    Number,
    String,
    Bool,
}

impl Type {
    pub fn new_builtin(ty: BuiltInType) -> Self {
        Type::BuiltIn(ty)
    }

    // this is expected to not be irrefutable when we add more types
    #[allow(irrefutable_let_patterns)] 
    pub fn as_builtin(&self) -> Option<&BuiltInType> {
        if let Type::BuiltIn(ty) = self {
            Some(ty)
        } else {
            None
        }
    }
}

pub fn to_string(ty: &TypeAnnotation) -> String {
    match ty {
        Some(Type::BuiltIn(ty)) => match ty {
            BuiltInType::Number => "number".to_string(),
            BuiltInType::String => "string".to_string(),
            BuiltInType::Bool => "bool".to_string(),
        },
        None => "none".to_string(),
    }
}
