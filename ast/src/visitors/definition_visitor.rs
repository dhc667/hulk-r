use crate::{ConstantDef, Definition, GlobalFunctionDef, ProtocolDef, TypeDef};

pub trait DefinitionVisitor<R> {
    fn visit_definition(&mut self, node: &mut Definition) -> R;

    fn visit_type_def(&mut self, node: &mut TypeDef) -> R;
    fn visit_function_def(&mut self, node: &mut GlobalFunctionDef) -> R;
    fn visit_constant_def(&mut self, node: &mut ConstantDef) -> R;
    fn visit_protocol_def(&mut self, node: &mut ProtocolDef) -> R;
}
