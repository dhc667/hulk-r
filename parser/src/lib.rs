use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

mod ast;
pub use ast::*;

pub mod tokens;
pub use tokens::*;

pub mod visitors;
pub use visitors::Visitor;
pub use visitors::Visitable;

pub use grammar::ProgramParser;

#[cfg(test)]
mod test {
    mod atom_parser;
    mod expression_parser;
    mod program_parser;
}
