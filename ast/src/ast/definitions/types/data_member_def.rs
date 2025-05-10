use crate::{Assignment, Identifier};
use crate::{Expression, tokens::BinaryOperator};

pub struct DataMemberDef {
    pub identifier: Identifier,
    pub assignment_op: BinaryOperator,
    pub default_value: Expression,
}

impl DataMemberDef {
    pub fn new(
        identifier: Identifier,
        assignment_op: BinaryOperator,
        default_value: Expression,
    ) -> DataMemberDef {
        DataMemberDef {
            identifier,
            assignment_op,
            default_value,
        }
    }
}

impl From<Assignment> for DataMemberDef {
    fn from(assignment: Assignment) -> Self {
        Self {
            identifier: assignment.identifier,
            assignment_op: assignment.op,
            default_value: *assignment.rhs,
        }
    }
}
