use std::fmt::Display;

use crate::regex_engine::regex_ast::regex_exp::RegexExp;

/// Represents a unary operator in a regular expression, such as Kleene star, plus, or optional.
/// This enum is used to define operations that can be applied to a regular expression operand.
/// # Variants
/// - `KleeneStar`: Represents the Kleene star operator (`*`), which matches zero or more occurrences of the operand.
/// - `Plus`: Represents the plus operator (`+`), which matches one or more occurrences of the operand.
/// - `Optional`: Represents the optional operator (`?`), which matches zero or one occurrence of the operand.
pub enum UnaryOperator {
    KleeneStar,
    Plus,
    Optional,
}

impl From<char> for UnaryOperator {
    fn from(c: char) -> Self {
        match c {
            '*' => UnaryOperator::KleeneStar,
            '+' => UnaryOperator::Plus,
            '?' => UnaryOperator::Optional,
            _ => panic!("Invalid unary operator character"),
        }
    }
}

/// Represents a unary operation in a regular expression, which consists of an operand and a unary operator.
/// This struct is used to encapsulate the operation applied to a regular expression operand.
/// # Fields
/// - `operand`: A boxed `RegexExp` representing the operand of the unary operation.
/// - `op`: A `UnaryOperator` that specifies the type of unary operation to be performed on the operand.
pub struct UnOp {
    pub operand: Box<RegexExp>,
    pub op: UnaryOperator,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            UnaryOperator::KleeneStar => write!(f, "{}*", self.operand),
            UnaryOperator::Plus => write!(f, "{}+", self.operand),
            UnaryOperator::Optional => write!(f, "{}?", self.operand),
        }
    }
}
