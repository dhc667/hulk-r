use ast::Program;
use lexer::lexer_generator::{lexer::Lexer, lexer_chunk::LexerChunk};
use parser_generator::{Parser, Token};

use crate::{
    grammar::lexer_parser,
    types::{ReturnType, TokenType},
};

pub struct ProgramParser {
    lexer: Lexer<TokenType>,
    parser: Parser<TokenType, ReturnType>,
}

impl ProgramParser {
    pub fn new() -> Self {
        let (lexer, parser) = lexer_parser();

        Self { lexer, parser }
    }

    pub fn parse(&self, input: &str) -> Result<Program, Vec<String>> {
        let tokens = self.lexer.split(input);
        if let Err((_, errors)) = tokens {
            return Err(errors);
        }
        let tokens = tokens.unwrap();
        let tokens = Self::lexer_output_to_token_vec(tokens);

        let parse = self.parser.parse(tokens);
        if let Err(err) = parse {
            let mut errors = Vec::new();
            let err = err.to_string(input);
            errors.push(err);

            return Err(errors);
        }

        Ok(parse.unwrap().try_into_program().unwrap())
    }

    fn lexer_output_to_token_vec(toks: Vec<LexerChunk<TokenType>>) -> Vec<Token<TokenType>> {
        toks.into_iter().map(Self::lexer_chunk_to_token).collect()
    }

    fn lexer_chunk_to_token(lexer_chunk: LexerChunk<TokenType>) -> Token<TokenType> {
        Token::new(
            lexer_chunk.ty,
            lexer_chunk.slice.to_string(),
            lexer_chunk.start,
            lexer_chunk.end,
        )
    }
}
