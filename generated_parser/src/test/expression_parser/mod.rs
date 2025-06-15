use ast::Expression;
use error_handler::error::error::HulkError;

use crate::ProgramParser;

mod additions;
mod concat;
mod logical;
mod terms;

mod block;
mod for_parser;
mod if_else;
mod let_in;
mod literal;
mod print;
mod unary_op;
mod while_parser;

mod lists;

mod functions;

mod data_member;
mod function_member;

mod destructive_assignment;

mod strings;

struct ExpressionParser {
    parser: ProgramParser,
}

impl ExpressionParser {
    fn new() -> Self {
        Self {
            parser: ProgramParser::new(),
        }
    }

    fn parse(&self, input: &str) -> Result<Expression, Vec<HulkError>> {
        self.parser
            .parse(&(input.to_string() + ";"))
            .map(|mut program| program.expressions.pop().unwrap())
    }
}
