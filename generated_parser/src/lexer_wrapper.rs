use error_handler::error::error::HulkError;
use lexer::lexer_generator::{lexer::Lexer, lexer_chunk::LexerChunk, rule::Rule};
use parser_generator::{DefineLexer, Lex, Token};

use crate::types::TokenType;

pub struct LexerWrapper {
    lexer: Lexer<TokenType>,
}

impl LexerWrapper {
    fn new(lexer: Lexer<TokenType>) -> Self {
        Self { lexer }
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

impl Lex<TokenType> for LexerWrapper {
    fn split(
        &self,
        input: &str,
    ) -> Result<Vec<parser_generator::Token<TokenType>>, Vec<HulkError>> {
        self.lexer
            .split(input)
            .map(|ok| ok.into_iter().map(Self::lexer_chunk_to_token).collect())
            .map_err(|(_, errs)| errs)
    }
}

pub struct LexerDefiner {
    rules: Vec<Rule<TokenType>>,
}

impl DefineLexer<TokenType, LexerWrapper> for LexerDefiner {
    fn new() -> Self {
        Self { rules: Vec::new() }
    }

    fn rule(&mut self, tok_ty: TokenType, pattern: String) {
        self.rules.push(Rule::new(tok_ty, pattern));
    }

    fn skip_rule(&mut self, tok_ty: TokenType, pattern: String) {
        self.rules.push(Rule::new_skip(tok_ty, pattern));
    }

    fn compile(self) -> LexerWrapper {
        LexerWrapper::new(Lexer::new(self.rules))
    }
}
