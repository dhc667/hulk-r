use crate::{
    DotOperator, Expression, ExpressionVisitor, FunctionCall, VisitableExpression,
    typing::TypeAnnotation,
};

#[derive(Debug)]
pub struct FunctionMemberAccess {
    pub object: Box<Expression>,
    pub obj_type: TypeAnnotation,
    pub op: DotOperator,
    pub member: FunctionCall,
}

impl FunctionMemberAccess {
    pub fn new(object: Expression, op: DotOperator, member: FunctionCall) -> Self {
        Self {
            object: Box::new(object),
            obj_type: None,
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
