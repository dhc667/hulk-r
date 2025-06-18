use std::fmt::Debug;

#[derive(Debug)]
pub enum ParseError<TokenType: Debug> {
    UnexpectedToken { ty: TokenType, loc: usize },
    UnexpectedEof,
}

impl<TokenType: Debug> ParseError<TokenType> {
    pub fn to_string(&self, input_program: &str) -> String {
        match self {
            Self::UnexpectedEof => "Unexpected end of file encountered".to_string(),
            Self::UnexpectedToken { ty, loc } => {
                Self::to_string_unexpected_token(input_program, ty, loc)
            }
        }
    }

    fn to_string_unexpected_token(input_program: &str, ty: &TokenType, loc: &usize) -> String {
        let newline_indices = input_program
            .char_indices()
            .filter(|(_, char)| char == &'\n')
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        let (line, column) = if newline_indices.is_empty() {
            (0, *loc)
        } else {
            let line = newline_indices.binary_search(loc);

            match line {
                Ok(idx) => (idx, 0),
                Err(idx) => (idx, loc - newline_indices.get(idx - 1).unwrap_or(&0)),
            }
        };

        format!("Unexpected token {:?} at {}:{}", ty, line, column)
    }
}
