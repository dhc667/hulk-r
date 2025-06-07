use crate::regex_engine::{
    automata::nfa_builder::NFABuilder,
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

    assert!(!nfa.simulate("ab".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
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

    assert!(nfa.simulate("ab".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
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

    assert!(nfa.simulate("acb".chars().collect()));
    assert!(!nfa.simulate("abaaaaaaasdsdsdadsfjaldsfaoisfuv;saivf;".chars().collect()));
    assert!(!nfa.simulate("ab".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
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

    assert!(nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("ab".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("c".chars().collect()));
    assert!(!nfa.simulate("acb".chars().collect()));
    assert!(!nfa.simulate("abaaaaaaboniatoboniatov;saivf;".chars().collect()));
    assert!(!nfa.simulate("abcc".chars().collect()));
}

#[test]
pub fn match_kleene_star_1() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::Symbol('a'.into()))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("aa".chars().collect()));
    assert!(nfa.simulate("aaaaaa".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
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

    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("ab".chars().collect()));
    assert!(nfa.simulate("abb".chars().collect()));
    assert!(nfa.simulate("abbb".chars().collect()));
    assert!(nfa.simulate("abbbbbbb".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("aa".chars().collect()));
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

    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("ab".chars().collect()));
    assert!(nfa.simulate("aab".chars().collect()));
    assert!(nfa.simulate("aaab".chars().collect()));
    assert!(nfa.simulate("aaaaaaab".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("aa".chars().collect()));
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

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("ab".chars().collect()));
    assert!(nfa.simulate("abab".chars().collect()));
    assert!(nfa.simulate("ababab".chars().collect()));
    assert!(nfa.simulate("abababab".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("aa".chars().collect()));
    assert!(!nfa.simulate("bb".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("aab".chars().collect()));
    assert!(!nfa.simulate("bba".chars().collect()));
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

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("ab".chars().collect()));
    assert!(nfa.simulate("aa".chars().collect()));
    assert!(nfa.simulate("bb".chars().collect()));
    assert!(nfa.simulate("abab".chars().collect()));
    assert!(nfa.simulate("aabab".chars().collect()));
    assert!(nfa.simulate("babab".chars().collect()));
    assert!(!nfa.simulate("c".chars().collect()));
    assert!(!nfa.simulate("ac".chars().collect()));
    assert!(!nfa.simulate("bc".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
    assert!(nfa.simulate("aab".chars().collect()));
    assert!(nfa.simulate("bba".chars().collect()));
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

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("abc".chars().collect()));
    assert!(nfa.simulate("xyz".chars().collect()));
    assert!(nfa.simulate("ABC".chars().collect()));
    assert!(nfa.simulate("XYZ".chars().collect()));
    assert!(nfa.simulate("AbC".chars().collect()));
    assert!(nfa.simulate("aBc".chars().collect()));
    assert!(nfa.simulate("aBcXyZ".chars().collect()));
    assert!(!nfa.simulate("1".chars().collect()));
    assert!(!nfa.simulate("!".chars().collect()));
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

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("0".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("123".chars().collect()));
    assert!(nfa.simulate("456789".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("!".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
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

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("c".chars().collect()));
    assert!(nfa.simulate("ab".chars().collect()));
    assert!(nfa.simulate("ac".chars().collect()));
    assert!(nfa.simulate("bc".chars().collect()));
    assert!(nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("d".chars().collect()));
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

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("!".chars().collect()));
    assert!(nfa.simulate("123".chars().collect()));
    assert!(!nfa.simulate("xyz".chars().collect()));
    assert!(!nfa.simulate("XYZ".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("ABC".chars().collect()));
    assert!(!nfa.simulate("AbC".chars().collect()));
    assert!(nfa.simulate("@#)*%@_#)($".chars().collect()));
    assert!(nfa.simulate("1234567890".chars().collect()));
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

    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("c".chars().collect()));
    assert!(!nfa.simulate("ab".chars().collect()));
    assert!(!nfa.simulate("aa".chars().collect()));
    assert!(!nfa.simulate("bb".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
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

    assert!(nfa.simulate("ab".chars().collect()));
    assert!(nfa.simulate("ac".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("c".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("aab".chars().collect()));
    assert!(!nfa.simulate("abb".chars().collect()));
    assert!(!nfa.simulate("aac".chars().collect()));
    assert!(!nfa.simulate("acc".chars().collect()));
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

    assert!(nfa.simulate("ac".chars().collect()));
    assert!(nfa.simulate("bc".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("c".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("ab".chars().collect()));
}

#[test]
pub fn match_dot() {
    let regex = RegexExp::Atom(MatchableSymbol::SymbolSet(SymbolSet::Dot));
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("c".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("!".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("".chars().collect()));
}

#[test]
pub fn match_dot_kleene_star() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(SymbolSet::Dot))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("c".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("!".chars().collect()));
    assert!(nfa.simulate("abc".chars().collect()));
    assert!(nfa.simulate("aab".chars().collect()));
    assert!(nfa.simulate("bba".chars().collect()));
    assert!(nfa.simulate("abacaba".chars().collect()));
    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate(" ".chars().collect()));
    assert!(nfa.simulate("  ".chars().collect()));
    assert!(nfa.simulate("   ".chars().collect()));
    assert!(nfa.simulate("a b c".chars().collect()));
    assert!(nfa.simulate("a b c d".chars().collect()));
    assert!(nfa.simulate("a b c d e".chars().collect()));
    assert!(nfa.simulate("a b c d e f".chars().collect()));
    assert!(nfa.simulate("a b c d e f g".chars().collect()));
    assert!(nfa.simulate("a b c d e f g h".chars().collect()));
}

#[test]
pub fn match_plus_1() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(SymbolSet::Dot))),
        op: UnaryOperator::Plus,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(!nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("c".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("!".chars().collect()));
    assert!(nfa.simulate("abc".chars().collect()));
    assert!(nfa.simulate("aab".chars().collect()));
    assert!(nfa.simulate("bba".chars().collect()));
    assert!(nfa.simulate("abacaba".chars().collect()));
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

    assert!(!nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("0".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("123".chars().collect()));
    assert!(nfa.simulate("456789".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("!".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_optional_1() {
    let regex = RegexExp::UnOp(UnOp {
        operand: Box::new(RegexExp::Atom(MatchableSymbol::SymbolSet(SymbolSet::Dot))),
        op: UnaryOperator::Optional,
    });
    let mut builder = NFABuilder::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("c".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("!".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
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

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("0".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("3".chars().collect()));
    assert!(!nfa.simulate("123".chars().collect()));
    assert!(!nfa.simulate("456789".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("!".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
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

    assert!(!nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("A".chars().collect()));
    assert!(nfa.simulate("abc".chars().collect()));
    assert!(nfa.simulate("ABC".chars().collect()));
    assert!(nfa.simulate("a1b2c3".chars().collect()));
    assert!(nfa.simulate("identifier123".chars().collect()));
    assert!(!nfa.simulate("1identifier".chars().collect()));
    assert!(!nfa.simulate("!@#".chars().collect()));
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

    assert!(nfa.simulate("123.456".chars().collect()));
    assert!(nfa.simulate("0.1".chars().collect()));
    assert!(nfa.simulate("0.5".chars().collect()));
    assert!(nfa.simulate("123.123123213461".chars().collect()));
    assert!(nfa.simulate("123.".chars().collect()));
    assert!(!nfa.simulate("Q_*#!@)$@!)N*V)_*@!N".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("123abc".chars().collect()));
}
