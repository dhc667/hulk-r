use super::{GeneratorVisitor, VisitorResult};
use ast;

impl GeneratorVisitor {
    pub(crate) fn _handle_for(
        &mut self,
        _element_name: &str,
        _iterable_result: VisitorResult,
        _node: &mut ast::For,
    ) -> VisitorResult {
        todo!()
    }
}
