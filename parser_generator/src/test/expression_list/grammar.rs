use crate::{
    Parser, Token, grammar,
    test::{LexerDefiner, LexerWrapper},
};

use crate::test::expression_list::{TokenType, return_type::ReturnType};

pub fn lexer_parser() -> (LexerWrapper<TokenType>, Parser<TokenType, ReturnType>) {
    let (lexer, parser) = grammar! {
        token_type: TokenType,
        return_type: ReturnType,
        lexer_definer_type: LexerDefiner,
        first_symbol: L,
        default_token_action: |tok: &Token<TokenType>| match tok.slice.parse::<i32>() {
            Ok(i) => ReturnType::Expression(i),
            Err(_) => ReturnType::Token,
        },

        productions: {
            L -> L Comma E = collect_L__L_Comma_E;
            L -> E = collect_L__E;
            E -> E Plus T = |mut v| ReturnType::Expression(
                v.remove(0).as_expression().unwrap() +
                v.remove(1).as_expression().unwrap()
            );
            E -> T = |mut v| v.remove(0);

            T -> T Times F = |mut v| ReturnType::Expression(
                v.remove(0).as_expression().unwrap() *
                v.remove(1).as_expression().unwrap()
            );
            T -> F = |mut v| v.remove(0);

            F -> Number = |mut v| v.remove(0);
            F -> Lpar E Rpar = |mut v| v.remove(1);
        }

        terminals: {
            (Number, r"[1-9][0-9]*"),
            (Plus, r"\+"),
            (Times, r"\*"),
            (Lpar, r"\("),
            (Rpar, r"\)"),
            (Comma, r",")
        }

        skip: {
            (__Whitespace__, r"\s+"),
        }
    };

    (lexer, parser)
}

#[allow(non_snake_case)]
fn collect_L__L_Comma_E(v: Vec<ReturnType>) -> ReturnType {
    let mut v = v;
    let l = v.remove(0);
    let e = v.remove(1);

    let mut l = l.as_expression_list().unwrap();

    l.push(e.as_expression().unwrap());

    return ReturnType::ExpressionList(l);
}

#[allow(non_snake_case)]
fn collect_L__E(v: Vec<ReturnType>) -> ReturnType {
    let mut v = v;
    let e = v.remove(0);

    let mut l = Vec::new();

    l.push(e.as_expression().unwrap());

    ReturnType::ExpressionList(l)
}
