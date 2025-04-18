use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

mod ast;
pub use ast::*;

pub mod tokens;
pub use tokens::*;

pub mod visitors;
pub use visitors::Visitor;

#[cfg(test)]
mod test {
    mod atom_parser;
    mod expression_parser;
}
