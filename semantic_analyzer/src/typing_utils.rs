use ast::{
    BinaryOperator, UnaryOperator,
    typing::{BuiltInType, Type, TypeAnnotation},
};

pub fn get_bin_op_return_type(op: &BinaryOperator) -> Type {
    match op {
        BinaryOperator::Plus(_) => Type::BuiltIn(BuiltInType::Number),
        BinaryOperator::Minus(_) => Type::BuiltIn(BuiltInType::Number),
        BinaryOperator::Divide(_) => Type::BuiltIn(BuiltInType::Number),
        BinaryOperator::FloorDivide(_) => Type::BuiltIn(BuiltInType::Number),
        BinaryOperator::Times(_) => Type::BuiltIn(BuiltInType::Number),
        BinaryOperator::Modulo(_) => Type::BuiltIn(BuiltInType::Number),
        BinaryOperator::EqualEqual(_) => Type::BuiltIn(BuiltInType::Bool),
        BinaryOperator::Less(_) => Type::BuiltIn(BuiltInType::Bool),
        BinaryOperator::LessEqual(_) => Type::BuiltIn(BuiltInType::Bool),
        BinaryOperator::Greater(_) => Type::BuiltIn(BuiltInType::Bool),
        BinaryOperator::GreaterEqual(_) => Type::BuiltIn(BuiltInType::Bool),
        BinaryOperator::NotEqual(_) => Type::BuiltIn(BuiltInType::Bool),
        BinaryOperator::Or(_) => Type::BuiltIn(BuiltInType::Bool),
        BinaryOperator::And(_) => Type::BuiltIn(BuiltInType::Bool),
        BinaryOperator::ColonEqual(_) => Type::BuiltIn(BuiltInType::Bool),
        BinaryOperator::Equal(_) => todo!(),
    }
}

pub fn get_up_op_return_type(op: &UnaryOperator) -> Type {
    match op {
        UnaryOperator::Plus(_) => Type::BuiltIn(BuiltInType::Number),
        UnaryOperator::Minus(_) => Type::BuiltIn(BuiltInType::Number),
    }
}

pub fn get_bin_op_param_type(op: &BinaryOperator) -> TypeAnnotation {
    match op {
        BinaryOperator::Plus(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::Minus(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::Divide(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::FloorDivide(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::Times(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::Modulo(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::EqualEqual(_) => None,
        BinaryOperator::Less(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::LessEqual(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::Greater(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::GreaterEqual(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        BinaryOperator::NotEqual(_) => None,
        BinaryOperator::Or(_) => Some(Type::BuiltIn(BuiltInType::Bool)),
        BinaryOperator::And(_) => Some(Type::BuiltIn(BuiltInType::Bool)),
        BinaryOperator::ColonEqual(_) => None,
        BinaryOperator::Equal(_) => None,
    }
}

pub fn get_up_op_param_type(op: &UnaryOperator) -> TypeAnnotation {
    match op {
        UnaryOperator::Plus(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        UnaryOperator::Minus(_) => Some(Type::BuiltIn(BuiltInType::Number)),
    }
}

pub fn is_bin_op_admisible(operand_type: &TypeAnnotation, op: &BinaryOperator) -> bool {
    match operand_type {
        None => true,
        Some(ty) => {
            let param_type = get_bin_op_param_type(op);
            match param_type {
                None => true,
                Some(param_type) => param_type == *ty,
            }
        }
    }
}

pub fn is_un_op_admisible(operand_type: &TypeAnnotation, op: &UnaryOperator) -> bool {
    match operand_type {
        None => true,
        Some(ty) => {
            let param_type = get_up_op_param_type(op);
            match param_type {
                None => true,
                Some(param_type) => param_type == *ty,
            }
        }
    }
}
