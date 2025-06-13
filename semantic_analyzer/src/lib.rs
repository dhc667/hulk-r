pub mod semantic_analyzer;

pub mod visitors;

pub mod typing;

pub mod graph_utils;

pub mod def_info;
#[cfg(test)]
pub mod test {
    pub mod definitions;
    pub mod goblal_definitions;
    pub mod inheritance;
    pub mod type_checking;
    pub mod type_definition;
}
