pub mod type_checker;
pub use type_checker::TypeChecker;
pub mod typing_utils;
pub use typing_utils::{get_binary_op_functor_type, get_unary_op_functor_type};
pub mod generics;
