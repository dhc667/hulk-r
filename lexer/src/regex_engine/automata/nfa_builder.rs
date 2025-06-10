use std::collections::{HashMap, HashSet};

use crate::regex_engine::{
    automata::nfa::NFA,
    regex_ast::{
        bin_op::{BinOp, BinaryOperator},
        regex_exp::RegexExp,
        symbol::{Symbol, symbol::MatchableSymbol},
        un_op::{UnOp, UnaryOperator},
    },
};

/// # Description
/// A builder for constructing a Non-deterministic Finite Automaton (NFA) from a regular expression.
pub struct NFABuilder {
    current_state: usize,
}

impl NFABuilder {
    pub fn new() -> Self {
        NFABuilder { current_state: 0 }
    }

    /// Constructs an NFA from a given regular expression.
    /// # Arguments
    /// - `regex`: A reference to a `RegexExp` instance representing the regular expression to be converted into an NFA.
    /// # Returns
    /// A new `NFA` instance that represents the equivalent NFA of the provided regular expression.
    pub fn build_from_regex(&mut self, regex: &RegexExp) -> NFA {
        match regex {
            RegexExp::Atom(symbol) => self.symbol(symbol),
            RegexExp::BinOp(bin_op) => self.bin_op(bin_op),
            RegexExp::UnOp(un_op) => self.un_op(un_op),
        }
    }

    /// Constructs an NFA from a given matchable symbol.
    /// # Arguments
    /// - `symbol`: A reference to a `MatchableSymbol` instance representing the symbol to be converted into an NFA.
    /// # Returns
    /// A new `NFA` instance that represents the equivalent NFA of the provided matchable symbol.
    fn symbol(&mut self, symbol: &MatchableSymbol) -> NFA {
        let q0 = self.current_state;
        let qf = self.current_state + 1;
        self.current_state += 2;

        let mut d = HashMap::new();

        match symbol {
            MatchableSymbol::SymbolSet(char_set) => {
                for a in 0u8..=127u8 {
                    let a = a as char;
                    if *char_set == a {
                        d.insert((q0, Symbol::Char(a)), HashSet::from([qf]));
                    }
                }
            }
            MatchableSymbol::Symbol(s) => {
                d.insert((q0, s.clone()), HashSet::from([qf]));
            }
        }
        NFA { q0, qf, d }
    }

    /// Builds an NFA from a binary or unary operation in a regular expression.
    /// # Arguments
    /// - `regex`: A reference to a `RegexExp` instance representing the operation to be converted into an NFA.
    /// # Returns
    /// A new `NFA` instance that represents the equivalent NFA of the provided operation.
    fn bin_op(&mut self, bin_op: &BinOp) -> NFA {
        let op = &bin_op.op;
        let nfa1 = self.build_from_regex(&bin_op.left);
        let nfa2 = self.build_from_regex(&bin_op.right);
        match op {
            &BinaryOperator::Concat => self.concat(&nfa1, &nfa2),
            &BinaryOperator::Union => self.union(&nfa1, &nfa2),
        }
    }

    /// Builds an NFA from a unary operation in a regular expression.
    /// # Arguments
    /// - `un_op`: A reference to a `UnOp` instance representing the unary operation to be converted into an NFA.
    /// # Returns
    /// A new `NFA` instance that represents the equivalent NFA of the provided unary operation.
    fn un_op(&mut self, un_op: &UnOp) -> NFA {
        let op = &un_op.op;
        let nfa = self.build_from_regex(&un_op.operand);
        match op {
            &UnaryOperator::KleeneStar => self.kleene_star(&nfa),
            &UnaryOperator::Plus => self.one_or_more(&nfa),
            &UnaryOperator::Optional => self.optional(&nfa),
        }
    }

    /// Constructs an NFA for concatenation of two NFAs.
    /// # Arguments
    /// - `nfa1`: A reference to the first `NFA` instance.
    /// - `nfa2`: A reference to the second `NFA` instance.
    /// # Returns
    /// A new `NFA` instance that represents the concatenation of the two NFAs.
    fn concat(&mut self, nfa1: &NFA, nfa2: &NFA) -> NFA {
        let q0 = nfa1.q0;
        let qf = nfa2.qf;

        let mut d = nfa1.d.clone();
        for ((q, c), p) in nfa2.d.iter() {
            if *q == nfa2.q0 {
                d.insert((nfa1.qf, c.clone()), p.clone());
            } else {
                d.insert((*q, c.clone()), p.clone());
            }
        }

        NFA { q0, qf, d }
    }

    /// Constructs an NFA for the union of two NFAs.
    /// # Arguments
    /// - `nfa1`: A reference to the first `NFA` instance.
    /// - `nfa2`: A reference to the second `NFA` instance.
    /// # Returns
    /// A new `NFA` instance that represents the union of the two NFAs.
    fn union(&mut self, nfa1: &NFA, nfa2: &NFA) -> NFA {
        let q0 = self.current_state;
        let qf = self.current_state + 1;
        self.current_state += 2;

        let mut transitions = HashMap::new();
        transitions.insert((q0, Symbol::Epsilon), HashSet::from([nfa1.q0, nfa2.q0]));
        transitions.insert((nfa1.qf, Symbol::Epsilon), HashSet::from([qf]));
        transitions.insert((nfa2.qf, Symbol::Epsilon), HashSet::from([qf]));

        for ((q, c), p) in nfa1.d.iter() {
            transitions.insert((*q, c.clone()), p.clone());
        }
        for ((q, c), p) in nfa2.d.iter() {
            transitions.insert((*q, c.clone()), p.clone());
        }

        NFA {
            q0,
            qf,
            d: transitions,
        }
    }

    /// Constructs an NFA for the Kleene star operation on a given NFA.
    /// # Arguments
    /// - `nfa`: A reference to the `NFA` instance to which the Kleene star operation will be applied.
    /// # Returns
    /// A new `NFA` instance that represents the Kleene star of the provided NFA.
    /// This operation allows the NFA to accept zero or more occurrences of the input string defined by the original NFA.
    fn kleene_star(&mut self, nfa: &NFA) -> NFA {
        let q0 = self.current_state;
        let qf = self.current_state + 1;
        self.current_state += 2;

        let mut d = HashMap::new();
        d.insert((q0, Symbol::Epsilon), HashSet::from([nfa.q0, qf]));
        d.insert((nfa.qf, Symbol::Epsilon), HashSet::from([nfa.q0, qf]));

        for ((q, c), p) in nfa.d.iter() {
            d.insert((*q, c.clone()), p.clone());
        }

        NFA { q0, qf, d }
    }

    /// Constructs an NFA for the one or more operation on a given NFA.
    /// # Arguments
    /// - `nfa`: A reference to the `NFA` instance to which the one or more operation will be applied.
    /// # Returns
    /// A new `NFA` instance that represents the one or more operation of the provided NFA.
    fn one_or_more(&mut self, nfa: &NFA) -> NFA {
        let q0 = self.current_state;
        let qf = self.current_state + 1;
        self.current_state += 2;

        let mut d = HashMap::new();
        d.insert((q0, Symbol::Epsilon), HashSet::from([nfa.q0]));
        d.insert((nfa.qf, Symbol::Epsilon), HashSet::from([nfa.q0, qf]));

        for ((q, c), p) in nfa.d.iter() {
            d.insert((*q, c.clone()), p.clone());
        }

        NFA { q0, qf, d }
    }

    /// Constructs an NFA for the optional operation on a given NFA.
    /// # Arguments
    /// - `nfa`: A reference to the `NFA` instance to which the optional operation will be applied.
    /// # Returns
    /// A new `NFA` instance that represents the optional operation of the provided NFA.
    /// This operation allows the NFA to accept either the input string defined by the original NFA or an empty string.
    fn optional(&mut self, nfa: &NFA) -> NFA {
        let q0 = nfa.q0;
        let qf = nfa.qf;
        let mut d = nfa.d.clone();

        let states = d.get_mut(&(q0, Symbol::Epsilon));
        if let Some(states) = states {
            states.insert(qf);
        } else {
            d.insert((q0, Symbol::Epsilon), HashSet::from([qf]));
        }
        NFA { q0, qf, d }
    }
}
