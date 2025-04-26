pub enum Type {
    BuiltIn(BuiltInType), //Defined(DefinedType),
}

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
