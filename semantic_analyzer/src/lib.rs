pub mod semantic_visitor;
pub use semantic_visitor::SemanticVisitor;

pub mod def_info;
pub use def_info::DefinitionInfo;

pub mod typing_utils;

#[cfg(test)]
pub mod test {
    pub mod definitions;
    pub mod type_checking;
}
