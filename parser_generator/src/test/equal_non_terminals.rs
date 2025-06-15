use crate::test::LexerDefiner;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum TokenType {
    A,
}

#[test]
#[should_panic]
fn equal_nts() {
    grammar! {
        token_type: TokenType,
        return_type: (),
        lexer_definer_type: LexerDefiner,
        first_symbol: S,
        default_token_action: |_: &_| {},

        productions: {
            S -> A = |_| {};
            S -> A = |_| {};
        }

        terminals: {
            (A, "A"),
        }

        skip: {}
    };
}
