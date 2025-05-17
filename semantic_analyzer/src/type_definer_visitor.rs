use ast::{DefinitionVisitor, VisitableDefinition, typing::Type};
use generator::context::Context;

pub struct TypeDefinerVisitor {
    /// # Description
    ///
    /// This is a visitor that defines types in the global context. It only looks at the names of the types. Setting the inheritance relationships
    /// between types and vitisiting fields and functions of the types is left for another visitor. This aims to solve the problem of recursive types,
    /// allowing the use of the type before it is defined, types that reference each other in a recursive manner, etc.
    pub definitions: Context<Type>,
    pub errors: Vec<String>,
}

impl TypeDefinerVisitor {
    pub fn new() -> Self {
        TypeDefinerVisitor {
            definitions: Context::new_one_frame(),
            errors: Vec::new(),
        }
    }
}

impl DefinitionVisitor<()> for TypeDefinerVisitor {
    fn visit_definition(&mut self, node: &mut ast::Definition) -> () {
        node.accept(self);
    }

    fn visit_type_def(&mut self, node: &mut ast::TypeDef) -> () {
        let type_name = node.name.id.clone();
        if self.definitions.is_defined(&type_name) {
            self.errors
                .push(format!("Type {} is already defined", type_name));
            return;
        }
        let type_def = Type::Defined(node.name.clone());
        self.definitions.define(type_name, type_def);
    }

    fn visit_function_def(&mut self, _node: &mut ast::GlobalFunctionDef) -> () {}

    fn visit_constant_def(&mut self, _node: &mut ast::ConstantDef) -> () {}

    fn visit_protocol_def(&mut self, _node: &mut ast::ProtocolDef) -> () {
        todo!()
    }
}
