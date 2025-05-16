use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub mod visitors;

pub use grammar::ProgramParser;

#[cfg(test)]
mod test {
    mod atom_parser;
    mod expression_parser;
    mod program_parser;
}
