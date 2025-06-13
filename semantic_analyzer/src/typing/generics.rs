use ast::typing::{FunctorType, Type, TypeAnnotation};

pub trait GenericType {
    fn generic_params(&self) -> Vec<TypeAnnotation>;
}

type IteratorType = Box<Type>;
impl GenericType for IteratorType {
    fn generic_params(&self) -> Vec<TypeAnnotation> {
        vec![Some(self.as_ref().clone())]
    }
}

impl GenericType for FunctorType {
    fn generic_params(&self) -> Vec<TypeAnnotation> {
        let mut generics = Vec::new();
        for param in &self.parameter_types {
            generics.push(param.clone());
        }
        generics.push(self.return_type.as_ref().clone());
        generics
    }
}
