pub mod semantic_visitor;
pub use semantic_visitor::SemanticVisitor;

pub mod def_context;
pub use def_context::DefContext;

pub mod hulk_type;
pub use hulk_type::{BuiltInType, Type};

pub mod variable_info;
pub use variable_info::VariableInfo;

#[cfg(test)]
pub mod test {
    mod contexts;
    mod definitions;
}
