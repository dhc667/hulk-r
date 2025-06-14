use ast::Program;
use error_handler::error::{
    error::HulkError,
    sintactic::{
        extra_token::ExtraTokenError, invalid_token::InvalidTokenError,
        unrecognized_eof::UnrecognizedEofError, unrecognized_token::UnrecognizedTokenError,
        user_error::UserError,
    },
};
use lalrpop_util::ParseError;

use crate::ProgramParser;

pub struct Parser {
    engine: ProgramParser,
}
impl Parser {
    pub fn new() -> Self {
        Parser {
            engine: ProgramParser::new(),
        }
    }

    pub fn parse(&self, input: &str) -> Result<Program, Vec<HulkError>> {
        let mut errors = Vec::<HulkError>::new();
        let result = self.engine.parse(input);
        match result {
            Ok(program) => Ok(program),
            Err(err) => match err {
                ParseError::InvalidToken { location } => {
                    errors.push(InvalidTokenError::new(location).into());
                    Err(errors)
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    errors.push(UnrecognizedEofError::new(expected, location).into());
                    Err(errors)
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    errors.push(
                        UnrecognizedTokenError::new(token.1.1.to_string(), expected, token.0)
                            .into(),
                    );
                    Err(errors)
                }
                ParseError::ExtraToken { token } => {
                    errors.push(ExtraTokenError::new(token.1.1.to_string(), token.0).into());
                    Err(errors)
                }
                ParseError::User { error } => {
                    errors.push(UserError::new(error.to_string(), 0).into());
                    Err(errors)
                }
            },
        }
    }
}
