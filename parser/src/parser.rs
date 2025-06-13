use ast::Program;
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

    pub fn parse(&self, input: &str) -> Result<Program, Vec<String>> {
        let mut errors = Vec::<String>::new();
        let result = self.engine.parse(input);
        match result {
            Ok(program) => Ok(program),
            Err(err) => match err {
                ParseError::InvalidToken { location } => {
                    errors.push(format!(
                        "Sintactic Error: Invalid token at location: {}",
                        location
                    ));
                    Err(errors)
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    let expected_clean: Vec<String> = expected
                        .into_iter()
                        .map(|s| s.replace('"', "`").replace("\\", ""))
                        .collect();
                    let expected_str = expected_clean.join(", ");
                    errors.push(format!(
                        "Sintactic Error: Unrecognized EOF at location: {}, expected: {}",
                        location, expected_str
                    ));
                    Err(errors)
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    let expected_clean: Vec<String> = expected
                        .into_iter()
                        .map(|s| s.replace('"', "`").replace("\\", ""))
                        .collect();
                    let expected_str = expected_clean.join(", ");
                    errors.push(format!(
                        "Sintactic Error: Unrecognized token at location: {}, token: `{}`, expected: {}",
                        token.0, token.1.1, expected_str
                    ));
                    Err(errors)
                }
                ParseError::ExtraToken { token } => {
                    errors.push(format!(
                        "Sintactic Error: Extra token at location: {}, token: {}",
                        token.0, token.1.1
                    ));
                    Err(errors)
                }
                ParseError::User { error } => {
                    errors.push(format!("Sintactic Error: {}", error));
                    Err(errors)
                }
            },
        }
    }
}
