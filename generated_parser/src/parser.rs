use ast::Program;
use error_handler::error::{error::HulkError, sintactic::user_error::UserError};
use parser_generator::{Lex, ParseError, Parser};

use crate::{
    grammar::lexer_parser,
    lexer_wrapper::LexerWrapper,
    types::{ReturnType, TokenType},
};

pub struct ProgramParser {
    lexer: LexerWrapper,
    parser: Parser<TokenType, ReturnType>,
}

impl ProgramParser {
    pub fn new() -> Self {
        let (lexer, parser) = lexer_parser();

        Self { lexer, parser }
    }

    pub fn parse(&self, input: &str) -> Result<Program, Vec<HulkError>> {
        let tokens = self.lexer.split(input)?;

        let parse = self.parser.parse(tokens);
        if let Err(err) = parse {
            let mut errors = Vec::new();
            let position = match err {
                ParseError::UnexpectedToken { loc, .. } => loc,
                ParseError::UnexpectedEof => {
                    if input.is_empty() {
                        0
                    } else {
                        input.len() - 1
                    }
                }
            };
            errors.push(UserError::new(err.to_string(input), position).into());

            return Err(errors);
        }

        Ok(parse.unwrap().try_into_program().unwrap())
    }
}
