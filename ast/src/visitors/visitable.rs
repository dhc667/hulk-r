use super::Visitor;

pub trait Visitable<T: Visitor<R>, R>{
    fn accept(&mut self, visitor: &mut T) -> R;
}
