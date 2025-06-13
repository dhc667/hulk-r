use crate::regex_engine::{
    automata::{dfa::DFA, nfa_builder::NFABuilder},
    regex_ast::{
        bin_op::{BinOp, BinaryOperator},
        regex_exp::RegexExp,
        symbol::{
            CharSet,
            symbol::{MatchableSymbol, SymbolSet},
        },
        un_op::{UnOp, UnaryOperator},
    },
};

#[test]
pub fn match_literal_1() {
    let regex = RegexExp::Atom(MatchableSymbol::Symbol('a'.into()));
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(!dfa.simulate("ab".chars().collect()));
    assert!(dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_literal_2() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
        right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("ab".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_literal_3() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::BinOp(BinOp {
            left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
            right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('c'.into()))),
            op: BinaryOperator::Concat,
        })),
        right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("acb".chars().collect()));
    assert!(!dfa.simulate("abaaaaaaasdsdsdadsfjaldsfaoisfuv;saivf;".chars().collect()));
    assert!(!dfa.simulate("ab".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_literal_4() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
        right: Box::new(RegexExp::BinOp(BinOp {
            left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
            right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('c'.into()))),
            op: BinaryOperator::Concat,
        })),
        op: BinaryOperator::Concat,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);
    assert!(dfa.simulate("abc".chars().collect()));
    assert!(!dfa.simulate("ab".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("c".chars().collect()));
    assert!(!dfa.simulate("acb".chars().collect()));
    assert!(!dfa.simulate("abaaaaaaboniatoboniatov;saivf;".chars().collect()));
    assert!(!dfa.simulate("abcc".chars().collect()));
}

#[test]
pub fn match_kleene_star_1() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("aa".chars().collect()));
    assert!(dfa.simulate("aaaaaa".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
}

#[test]
pub fn match_kleene_star_2() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
        right: Box::new(RegexExp::UnOp(UnOp {
            operand: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
            op: UnaryOperator::KleeneStar,
        })),
        op: BinaryOperator::Concat,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("ab".chars().collect()));
    assert!(dfa.simulate("abb".chars().collect()));
    assert!(dfa.simulate("abbb".chars().collect()));
    assert!(dfa.simulate("abbbbbbb".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("aa".chars().collect()));
}

#[test]
pub fn match_kleene_star_3() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::UnOp(UnOp {
            operand: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
            op: UnaryOperator::KleeneStar,
        })),
        right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("b".chars().collect()));
    assert!(dfa.simulate("ab".chars().collect()));
    assert!(dfa.simulate("aab".chars().collect()));
    assert!(dfa.simulate("aaab".chars().collect()));
    assert!(dfa.simulate("aaaaaaab".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("aa".chars().collect()));
}

#[test]
pub fn match_kleene_star_4() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::BinOp(BinOp {
            left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
            right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
            op: BinaryOperator::Concat,
        })),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("ab".chars().collect()));
    assert!(dfa.simulate("abab".chars().collect()));
    assert!(dfa.simulate("ababab".chars().collect()));
    assert!(dfa.simulate("abababab".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("aa".chars().collect()));
    assert!(!dfa.simulate("bb".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
    assert!(!dfa.simulate("aab".chars().collect()));
    assert!(!dfa.simulate("bba".chars().collect()));
}

#[test]
pub fn match_kleene_star_5() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::BinOp(BinOp {
            left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
            right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
            op: BinaryOperator::Union,
        })),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("b".chars().collect()));
    assert!(dfa.simulate("ab".chars().collect()));
    assert!(dfa.simulate("aa".chars().collect()));
    assert!(dfa.simulate("bb".chars().collect()));
    assert!(dfa.simulate("abab".chars().collect()));
    assert!(dfa.simulate("aabab".chars().collect()));
    assert!(dfa.simulate("babab".chars().collect()));
    assert!(!dfa.simulate("c".chars().collect()));
    assert!(!dfa.simulate("ac".chars().collect()));
    assert!(!dfa.simulate("bc".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
    assert!(dfa.simulate("aab".chars().collect()));
    assert!(dfa.simulate("bba".chars().collect()));
}

#[test]
pub fn match_alpha() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
            SymbolSet::CharSet(CharSet {
                ranges: vec![('a', 'z'), ('A', 'Z')],
                negated: false,
            }),
        ))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("b".chars().collect()));
    assert!(dfa.simulate("abc".chars().collect()));
    assert!(dfa.simulate("xyz".chars().collect()));
    assert!(dfa.simulate("ABC".chars().collect()));
    assert!(dfa.simulate("XYZ".chars().collect()));
    assert!(dfa.simulate("AbC".chars().collect()));
    assert!(dfa.simulate("aBc".chars().collect()));
    assert!(dfa.simulate("aBcXyZ".chars().collect()));
    assert!(!dfa.simulate("1".chars().collect()));
    assert!(!dfa.simulate("!".chars().collect()));
}

#[test]
pub fn match_digit() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
            SymbolSet::CharSet(CharSet {
                ranges: vec![('0', '9')],
                negated: false,
            }),
        ))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("0".chars().collect()));
    assert!(dfa.simulate("1".chars().collect()));
    assert!(dfa.simulate("123".chars().collect()));
    assert!(dfa.simulate("456789".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("!".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_char_set_1() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
            SymbolSet::CharSet(CharSet {
                ranges: vec![('a', 'c')],
                negated: false,
            }),
        ))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("b".chars().collect()));
    assert!(dfa.simulate("c".chars().collect()));
    assert!(dfa.simulate("ab".chars().collect()));
    assert!(dfa.simulate("ac".chars().collect()));
    assert!(dfa.simulate("bc".chars().collect()));
    assert!(dfa.simulate("abc".chars().collect()));
    assert!(!dfa.simulate("d".chars().collect()));
}

#[test]
pub fn match_non_alpha() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
            SymbolSet::CharSet(CharSet {
                ranges: vec![('a', 'z'), ('A', 'Z')],
                negated: true,
            }),
        ))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("1".chars().collect()));
    assert!(dfa.simulate("!".chars().collect()));
    assert!(dfa.simulate("123".chars().collect()));
    assert!(!dfa.simulate("xyz".chars().collect()));
    assert!(!dfa.simulate("XYZ".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
    assert!(!dfa.simulate("ABC".chars().collect()));
    assert!(!dfa.simulate("AbC".chars().collect()));
    assert!(dfa.simulate("@#)*%@_#)($".chars().collect()));
    assert!(dfa.simulate("1234567890".chars().collect()));
}

#[test]
pub fn match_union_1() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
        right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
        op: BinaryOperator::Union,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("c".chars().collect()));
    assert!(!dfa.simulate("ab".chars().collect()));
    assert!(!dfa.simulate("aa".chars().collect()));
    assert!(!dfa.simulate("bb".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_union_2() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
        right: Box::new(RegexExp::BinOp(BinOp {
            left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
            right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('c'.into()))),
            op: BinaryOperator::Union,
        })),
        op: BinaryOperator::Concat,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);
    assert!(dfa.simulate("ab".chars().collect()));
    assert!(dfa.simulate("ac".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("c".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
    assert!(!dfa.simulate("aab".chars().collect()));
    assert!(!dfa.simulate("abb".chars().collect()));
    assert!(!dfa.simulate("aac".chars().collect()));
    assert!(!dfa.simulate("acc".chars().collect()));
}

#[test]
pub fn match_union_3() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::BinOp(BinOp {
            left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
            right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('b'.into()))),
            op: BinaryOperator::Union,
        })),
        right: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('c'.into()))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("ac".chars().collect()));
    assert!(dfa.simulate("bc".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("b".chars().collect()));
    assert!(!dfa.simulate("c".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
    assert!(!dfa.simulate("ab".chars().collect()));
}

#[test]
pub fn match_dot() {
    let regex = RegexExp::Atom(MatchableSymbol::SymbolSet(SymbolSet::Dot));
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("b".chars().collect()));
    assert!(dfa.simulate("c".chars().collect()));
    assert!(dfa.simulate("1".chars().collect()));
    assert!(dfa.simulate("!".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
    assert!(!dfa.simulate("".chars().collect()));
}

#[test]
pub fn match_dot_kleene_star() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(SymbolSet::Dot))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("b".chars().collect()));
    assert!(dfa.simulate("c".chars().collect()));
    assert!(dfa.simulate("1".chars().collect()));
    assert!(dfa.simulate("!".chars().collect()));
    assert!(dfa.simulate("abc".chars().collect()));
    assert!(dfa.simulate("aab".chars().collect()));
    assert!(dfa.simulate("bba".chars().collect()));
    assert!(dfa.simulate("abacaba".chars().collect()));
    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate(" ".chars().collect()));
    assert!(dfa.simulate("  ".chars().collect()));
    assert!(dfa.simulate("   ".chars().collect()));
    assert!(dfa.simulate("a b c".chars().collect()));
    assert!(dfa.simulate("a b c d".chars().collect()));
    assert!(dfa.simulate("a b c d e".chars().collect()));
    assert!(dfa.simulate("a b c d e f".chars().collect()));
    assert!(dfa.simulate("a b c d e f g".chars().collect()));
    assert!(dfa.simulate("a b c d e f g h".chars().collect()));
}

#[test]
pub fn match_plus_1() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(SymbolSet::Dot))),
        op: UnaryOperator::Plus,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(!dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("b".chars().collect()));
    assert!(dfa.simulate("c".chars().collect()));
    assert!(dfa.simulate("1".chars().collect()));
    assert!(dfa.simulate("!".chars().collect()));
    assert!(dfa.simulate("abc".chars().collect()));
    assert!(dfa.simulate("aab".chars().collect()));
    assert!(dfa.simulate("bba".chars().collect()));
    assert!(dfa.simulate("abacaba".chars().collect()));
}

#[test]
pub fn match_plus_2() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
            SymbolSet::CharSet(CharSet {
                ranges: vec![('0', '9')],
                negated: false,
            }),
        ))),
        op: UnaryOperator::Plus,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(!dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("0".chars().collect()));
    assert!(dfa.simulate("1".chars().collect()));
    assert!(dfa.simulate("123".chars().collect()));
    assert!(dfa.simulate("456789".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("!".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_optional_1() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(SymbolSet::Dot))),
        op: UnaryOperator::Optional,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("b".chars().collect()));
    assert!(dfa.simulate("c".chars().collect()));
    assert!(dfa.simulate("1".chars().collect()));
    assert!(dfa.simulate("!".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_optional_2() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
            SymbolSet::CharSet(CharSet {
                ranges: vec![('0', '9')],
                negated: false,
            }),
        ))),
        op: UnaryOperator::Optional,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("0".chars().collect()));
    assert!(dfa.simulate("1".chars().collect()));
    assert!(dfa.simulate("3".chars().collect()));
    assert!(!dfa.simulate("123".chars().collect()));
    assert!(!dfa.simulate("456789".chars().collect()));
    assert!(!dfa.simulate("a".chars().collect()));
    assert!(!dfa.simulate("!".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_identifier() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
            SymbolSet::CharSet(CharSet {
                ranges: vec![('a', 'z'), ('A', 'Z')],
                negated: false,
            }),
        ))),
        right: Box::new(RegexExp::UnOp(UnOp {
            operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
                SymbolSet::CharSet(CharSet {
                    ranges: vec![('0', '9'), ('a', 'z'), ('A', 'Z')],
                    negated: false,
                }),
            ))),
            op: UnaryOperator::KleeneStar,
        })),
        op: BinaryOperator::Concat,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(!dfa.simulate("".chars().collect()));
    assert!(dfa.simulate("a".chars().collect()));
    assert!(dfa.simulate("A".chars().collect()));
    assert!(dfa.simulate("abc".chars().collect()));
    assert!(dfa.simulate("ABC".chars().collect()));
    assert!(dfa.simulate("a1b2c3".chars().collect()));
    assert!(dfa.simulate("identifier123".chars().collect()));
    assert!(!dfa.simulate("1identifier".chars().collect()));
    assert!(!dfa.simulate("!@#".chars().collect()));
}

#[test]
pub fn match_float() {
    let regex = RegexExp::BinOp(BinOp {
        left: Box::new(RegexExp::UnOp(UnOp {
            operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
                SymbolSet::CharSet(CharSet {
                    ranges: vec![('0', '9')],
                    negated: false,
                }),
            ))),
            op: UnaryOperator::KleeneStar,
        })),
        right: Box::new(RegexExp::BinOp(BinOp {
            left: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('.'.into()))),
            right: Box::new(RegexExp::UnOp(UnOp {
                operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(
                    SymbolSet::CharSet(CharSet {
                        ranges: vec![('0', '9')],
                        negated: false,
                    }),
                ))),
                op: UnaryOperator::KleeneStar,
            })),
            op: BinaryOperator::Concat,
        })),
        op: BinaryOperator::Concat,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);
    let dfa = DFA::from(nfa);

    assert!(dfa.simulate("123.456".chars().collect()));
    assert!(dfa.simulate("0.1".chars().collect()));
    assert!(dfa.simulate("0.5".chars().collect()));
    assert!(dfa.simulate("123.123123213461".chars().collect()));
    assert!(dfa.simulate("123.".chars().collect()));
    assert!(!dfa.simulate("Q_*#!@)$@!)N*V)_*@!N".chars().collect()));
    assert!(!dfa.simulate("abc".chars().collect()));
    assert!(!dfa.simulate("123abc".chars().collect()));
}
