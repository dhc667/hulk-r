use std::fmt::Display;

use crate::regex_engine::regex_ast::regex_exp::RegexExp;

/// Represents a binary operator in a regular expression, such as concatenation or union.
/// This enum is used to define operations that can be applied to two regular expression operands.
/// # Variants
/// - `Concat`: Represents the concatenation operator, which combines two regular expressions in sequence.
/// - `Union`: Represents the union operator, which matches either of the two regular expressions.
/// This enum is used to define operations that can be applied to two regular expression operands.
pub enum BinaryOperator {
    Concat,
    Union,
}

/// Represents a binary operation in a regular expression, which consists of two operands and a binary operator.
/// This struct is used to encapsulate the operation applied to two regular expression operands.
/// # Fields
/// - `left`: A boxed `RegexExp` representing the left operand of the binary operation.
/// - `right`: A boxed `RegexExp` representing the right operand of the binary operation.
/// - `op`: A `BinaryOperator` that specifies the type of binary operation to be performed on the operands.
/// This struct is used to encapsulate the operation applied to two regular expression operands.
/// # Fields
/// - `left`: A boxed `RegexExp` representing the left operand of the binary operation.
/// - `right`: A boxed `RegexExp` representing the right operand of the binary operation.
/// - `op`: A `BinaryOperator` that specifies the type of binary operation to be performed on the operands.
/// This struct is used to encapsulate the operation applied to two regular expression operands.
pub struct BinOp {
    pub left: Box<RegexExp>,
    pub right: Box<RegexExp>,
    pub op: BinaryOperator,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            BinaryOperator::Concat => write!(f, "({}{})", self.left, self.right),
            BinaryOperator::Union => write!(f, "({}|{})", self.left, self.right),
        }
    }
}
