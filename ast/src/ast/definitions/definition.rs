use crate::{DefinitionVisitor, VisitableDefinition};

use super::{
    ProtocolDef, constants::ConstantDef, global_functions::GlobalFunctionDef, types::TypeDef,
};

#[derive(Debug)]
pub enum Definition {
    TypeDef(TypeDef),
    FunctionDef(GlobalFunctionDef),
    ConstantDef(ConstantDef),
    ProtocolDef(ProtocolDef),
}

impl Definition {
    pub fn as_type_def(&self) -> Option<&TypeDef> {
        if let Self::TypeDef(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_function_def(&self) -> Option<&GlobalFunctionDef> {
        if let Self::FunctionDef(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_constant_def(&self) -> Option<&ConstantDef> {
        if let Self::ConstantDef(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_protocol_def(&self) -> Option<&ProtocolDef> {
        if let Self::ProtocolDef(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<ProtocolDef> for Definition {
    fn from(v: ProtocolDef) -> Self {
        Self::ProtocolDef(v)
    }
}

impl From<ConstantDef> for Definition {
    fn from(v: ConstantDef) -> Self {
        Self::ConstantDef(v)
    }
}

impl From<GlobalFunctionDef> for Definition {
    fn from(v: GlobalFunctionDef) -> Self {
        Self::FunctionDef(v)
    }
}

impl From<TypeDef> for Definition {
    fn from(v: TypeDef) -> Self {
        Self::TypeDef(v)
    }
}

impl<T: DefinitionVisitor<R>, R> VisitableDefinition<T, R> for Definition {
    fn accept(&mut self, visitor: &mut T) -> R {
        match self {
            Self::TypeDef(v) => visitor.visit_type_def(v),
            Self::FunctionDef(v) => visitor.visit_function_def(v),
            Self::ConstantDef(v) => visitor.visit_constant_def(v),
            Self::ProtocolDef(v) => visitor.visit_protocol_def(v),
        }
    }
}
