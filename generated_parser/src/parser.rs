use ast::Program;
use parser_generator::{Lex, Parser};

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

    pub fn parse(&self, input: &str) -> Result<Program, Vec<String>> {
        let tokens = self.lexer.split(input)?;

        let parse = self.parser.parse(tokens);
        if let Err(err) = parse {
            let mut errors = Vec::new();
            let err = err.to_string(input);
            errors.push(err);

            return Err(errors);
        }

        Ok(parse.unwrap().try_into_program().unwrap())
    }
}
