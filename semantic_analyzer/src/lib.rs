pub(crate) mod semantic_visitor;
pub(crate) use semantic_visitor::SemanticVisitor;

pub mod semantic_analyzer;

pub mod def_info;
pub use def_info::DefinitionInfo;

pub mod typing_utils;

pub mod type_definer_visitor;

#[cfg(test)]
pub mod test {
    pub mod definitions;
    pub mod goblal_definitions;
    pub mod type_checking;
}
