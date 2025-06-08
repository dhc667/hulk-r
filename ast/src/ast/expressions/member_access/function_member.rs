use crate::{
    DotOperator, Expression, ExpressionVisitor, FunctionCall, VisitableExpression,
    typing::TypeAnnotation,
};

pub struct FunctionMemberAccess {
    pub object: Box<Expression>,
    pub obj_type: TypeAnnotation,
    pub op: DotOperator,
    pub member: FunctionCall,
    pub resolved_type: TypeAnnotation,
}

impl FunctionMemberAccess {
    pub fn new(object: Expression, op: DotOperator, member: FunctionCall) -> Self {
        Self {
            object: Box::new(object),
            obj_type: None,
            op,
            member,
            resolved_type: None,
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for FunctionMemberAccess {
    fn accept(&mut self, visitor: &mut T) -> R {
        visitor.visit_function_member_access(self)
    }
}
