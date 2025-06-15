use crate::{
    Token,
    test::{LexerDefiner, non_lalr_grammar::token_type::TokenType},
};

#[test]
#[should_panic]
// Compilers Principles, Techniques and Tools second edition pp 267
pub fn lexer_parser() {
    grammar! {
        token_type: TokenType,
        return_type: (),
        lexer_definer_type: LexerDefiner,
        first_symbol: S,
        default_token_action: |tok: &Token<TokenType>| {
            eprintln!("Parsed token {:?}", tok.ty);
        },

        productions: {
            S -> a A d = |_| {};
            S -> b B d = |_| {};
            S -> a B e = |_| {};
            S -> b A e = |_| {};
            A -> c = |_| {};
            B -> c = |_| {};
        }

        terminals: {
            (a, "a"),
            (b, "b"),
            (c, "c"),
            (d, "d"),
            (e, "e"),
        }

        skip: {}
    };
}
