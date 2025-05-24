use ast::{FunctionDef, Identifier, typing::FunctorType};

#[derive(Clone)]
pub struct FuncInfo {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub functor_type: FunctorType,
}

impl FuncInfo {
    pub fn new(name: Identifier, parameters: Vec<Identifier>) -> Self {
        let functor_type = FunctorType::new(
            parameters.iter().map(|id| id.info.ty.clone()).collect(),
            None,
        );

        Self {
            name,
            parameters,
            functor_type,
        }
    }

    pub fn get_type_wrapper_name(info: &FuncInfo) -> String {
        format!("${}TypeWrapper", info.name.clone())
    }

    pub fn get_var_instance_name(info: &FuncInfo) -> String {
        format!("${}Instance", info.name.clone())
    }
}

impl From<&FunctionDef> for FuncInfo {
    fn from(func_def: &FunctionDef) -> Self {
        let parameters = func_def.parameters.clone();
        Self {
            name: func_def.identifier.clone(),
            parameters,
            functor_type: FunctorType::new(
                func_def
                    .parameters
                    .iter()
                    .map(|id| id.info.ty.clone())
                    .collect(),
                func_def.identifier.info.ty.clone(),
            ),
        }
    }
}
