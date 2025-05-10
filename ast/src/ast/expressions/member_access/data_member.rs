use crate::{DotOperator, Expression, ExpressionVisitor, Identifier, VisitableExpression};

pub struct DataMemberAccess {
    pub object: Box<Expression>,
    pub op: DotOperator,
    pub member: Identifier,
}

impl DataMemberAccess {
    pub fn new(object: Expression, op: DotOperator, member: Identifier) -> Self {
        Self {
            object: Box::new(object),
            op,
            member,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for DataMemberAccess {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_data_member_access(self)
    }
}
