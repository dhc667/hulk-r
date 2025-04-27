#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    BuiltIn(BuiltInType), //Defined(DefinedType),
}

#[derive(Clone, PartialEq, Debug)]
pub enum BuiltInType {
    Number,
    String,
    Bool,
}

impl Type {
    pub fn new_builtin(ty: BuiltInType) -> Self {
        Type::BuiltIn(ty)
    }
    pub fn as_builtin(&self) -> Option<&BuiltInType> {
        let Type::BuiltIn(ty) = self;
        Some(ty)
    }
}

pub type TypeAnnotation = Option<Type>;

pub fn convert_to_string(ty: &TypeAnnotation) -> String {
    match ty {
        Some(Type::BuiltIn(ty)) => match ty {
            BuiltInType::Number => "number".to_string(),
            BuiltInType::String => "string".to_string(),
            BuiltInType::Bool => "bool".to_string(),
        },
        None => "none".to_string(),
    }
}
