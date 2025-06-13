use ast::{
    BinaryOperator, UnaryOperator,
    typing::{BuiltInType, FunctorType, Type},
};

/// # Description
/// Returns the `FunctorType` for a given binary operator.
/// # Arguments
/// * `op` - The binary operator for which to get the functor type.
/// # Returns
/// A `FunctorType` representing the input and output types of the binary operator.
pub fn get_binary_op_functor_type(op: &BinaryOperator) -> FunctorType {
    match op {
        // Arithmetic
        BinaryOperator::Plus(_)
        | BinaryOperator::Minus(_)
        | BinaryOperator::Divide(_)
        | BinaryOperator::FloorDivide(_)
        | BinaryOperator::Times(_)
        | BinaryOperator::Modulo(_) => FunctorType::new(
            vec![
                Some(Type::BuiltIn(BuiltInType::Number)),
                Some(Type::BuiltIn(BuiltInType::Number)),
            ],
            Some(Type::BuiltIn(BuiltInType::Number)),
        ),

        // Comparison
        BinaryOperator::Less(_)
        | BinaryOperator::LessEqual(_)
        | BinaryOperator::Greater(_)
        | BinaryOperator::GreaterEqual(_) => FunctorType::new(
            vec![
                Some(Type::BuiltIn(BuiltInType::Number)),
                Some(Type::BuiltIn(BuiltInType::Number)),
            ],
            Some(Type::BuiltIn(BuiltInType::Bool)),
        ),

        // Equality
        BinaryOperator::EqualEqual(_) | BinaryOperator::NotEqual(_) => FunctorType::new(
            vec![
                Some(Type::BuiltIn(BuiltInType::Object)),
                Some(Type::BuiltIn(BuiltInType::Object)),
            ],
            Some(Type::BuiltIn(BuiltInType::Bool)),
        ),

        // Logical
        BinaryOperator::Or(_) | BinaryOperator::And(_) => FunctorType::new(
            vec![
                Some(Type::BuiltIn(BuiltInType::Bool)),
                Some(Type::BuiltIn(BuiltInType::Bool)),
            ],
            Some(Type::BuiltIn(BuiltInType::Bool)),
        ),

        // Assignment
        BinaryOperator::Equal(_) | BinaryOperator::ColonEqual(_) => FunctorType::new(
            vec![
                Some(Type::BuiltIn(BuiltInType::Object)),
                Some(Type::BuiltIn(BuiltInType::Object)),
            ],
            Some(Type::BuiltIn(BuiltInType::Object)),
        ),
    }
}

/// # Description
/// Returns the `FunctorType` for a given unary operator.
/// # Arguments
/// * `op` - The unary operator for which to get the functor type.
/// # Returns
/// A `FunctorType` representing the input and output types of the unary operator.
pub fn get_unary_op_functor_type(op: &UnaryOperator) -> FunctorType {
    match op {
        UnaryOperator::Plus(_) | UnaryOperator::Minus(_) => FunctorType::new(
            vec![Some(Type::BuiltIn(BuiltInType::Number))],
            Some(Type::BuiltIn(BuiltInType::Number)),
        ),
        UnaryOperator::Not(_) => FunctorType::new(
            vec![Some(Type::BuiltIn(BuiltInType::Bool))],
            Some(Type::BuiltIn(BuiltInType::Bool)),
        ),
    }
}
