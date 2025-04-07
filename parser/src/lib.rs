use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

//  TODO: Separate the AST into different files

// mod ast {
//     pub mod expression;
//     pub mod term;
//     pub mod addition;
//     pub mod atom;
//     pub mod factor;
//     pub mod identifier;
//     pub mod operator;
//     pub mod number_literal;
//     pub mod let_in_expression;
//     pub mod if_else_expression;
//     pub mod while_expression;
// }
//
mod ast;


#[cfg(test)]
mod test {
    mod atom_parser;
    mod expression_parser;
    mod term_parser;
    mod addition_parser;
}
