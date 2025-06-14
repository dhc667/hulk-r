use lexer::lexer_generator::{lexer::Lexer, rule::Rule};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum TokenType {
    A,
}

#[test]
#[should_panic(expected = "Non terminal B has no productions associated to it")]
fn undefined_nt() {
    grammar! {
        token_type: TokenType,
        return_type: (),
        lexer_type: Lexer,
        rule_type: Rule,
        first_symbol: S,
        default_token_action: |_: &_| {},

        productions: {
            S -> S A B = |_| {};
            S -> A B = |_| {};
        }

        terminals: {
            (A, "A"),
        }
    };
}
