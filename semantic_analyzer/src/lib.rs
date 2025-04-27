pub mod semantic_visitor;
pub use semantic_visitor::SemanticVisitor;

pub mod def_context;
pub use def_context::DefContext;

pub mod hulk_type;
pub use hulk_type::{BuiltInType, Type, TypeAnnotation, convert_to_string};

pub mod variable_info;
pub use variable_info::VariableInfo;

pub mod type_checker_visitor;
pub use type_checker_visitor::TypeCheckerVisitor;

#[cfg(test)]
pub mod test {
    mod contexts;
    mod definitions;
    mod type_checking;
}
