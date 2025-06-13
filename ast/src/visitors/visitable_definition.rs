use super::DefinitionVisitor;

pub trait VisitableDefinition<T: DefinitionVisitor<R>, R> {
    fn accept(&mut self, visitor: &mut T) -> R;
}
