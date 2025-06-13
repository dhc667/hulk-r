use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub mod visitors;

mod parsing_helpers;

pub use grammar::ProgramParser;

pub mod parser;

#[cfg(test)]
mod test {
    mod expression_parser;
    mod parsing_errors;
    mod program_parser;
}
