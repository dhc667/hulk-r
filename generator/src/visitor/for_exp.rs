use super::{GeneratorVisitor, VisitorResult};
use crate::llvm_types::{LlvmType};
use ast;

impl GeneratorVisitor {
    pub(crate) fn handle_for(
        &mut self,
        element_name: &str,
        iterable_result: VisitorResult,
        node: &mut ast::For,
    ) -> VisitorResult {
        todo!()
    }
}