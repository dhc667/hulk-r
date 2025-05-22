pub(crate) mod semantic_visitor;
pub(crate) use semantic_visitor::SemanticVisitor;

pub mod semantic_analyzer;

pub mod def_info;
pub use def_info::DefinitionInfo;

pub mod typing_utils;

pub mod type_definer_visitor;
pub use type_definer_visitor::TypeDefinerVisitor;

pub mod inheritance_visitor;
pub use inheritance_visitor::InheritanceVisitor;

pub mod type_checker;
pub use type_checker::TypeChecker;

pub mod lca;

pub mod type_info;
pub use type_info::DefinedTypeInfo;
pub use type_info::TypeInfo;

pub mod func_info;
pub use func_info::FuncInfo;
#[cfg(test)]
pub mod test {
    pub mod definitions;
    pub mod goblal_definitions;
    pub mod inheritance;
    pub mod type_checking;
    pub mod type_definition;
}
