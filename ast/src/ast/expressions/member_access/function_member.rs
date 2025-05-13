use crate::{DotOperator, Expression, ExpressionVisitor, FunctionCall, VisitableExpression};

pub struct FunctionMemberAccess {
    pub object: Box<Expression>,
    pub op: DotOperator,
    pub member: FunctionCall,
}

impl FunctionMemberAccess {
    pub fn new(object: Expression, op: DotOperator, member: FunctionCall) -> Self {
        Self {
            object: Box::new(object),
            op,
            member,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for FunctionMemberAccess {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_function_member_access(self)
    }
}
