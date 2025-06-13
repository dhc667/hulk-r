use super::ExpressionVisitor;

pub trait VisitableExpression<T: ExpressionVisitor<R>, R> {
    fn accept(&mut self, visitor: &mut T) -> R;
}
