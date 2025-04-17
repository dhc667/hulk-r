use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

mod ast;
pub mod tokens;

#[cfg(test)]
mod test {
    mod atom_parser;
    mod expression_parser;
}
