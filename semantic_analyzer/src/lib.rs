pub(crate) mod semantic_visitor;
pub(crate) use semantic_visitor::SemanticVisitor;

pub mod semantic_analyzer;

pub mod def_info;
pub use def_info::DefinitionInfo;

pub mod typing_utils;

#[cfg(test)]
pub mod test {
    pub mod definitions;
    pub mod type_checking;
}
