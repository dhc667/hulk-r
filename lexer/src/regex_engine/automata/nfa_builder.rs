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

pub struct NFABuilder {
    current_state: usize,
}

impl NFABuilder {
    pub fn new() -> Self {
        NFABuilder { current_state: 0 }
    }

    pub fn build_from_regex(&mut self, regex: &RegexExp) -> NFA {
        match regex {
            RegexExp::Atom(symbol) => self.symbol(symbol),
            RegexExp::BinOp(bin_op) => self.bin_op(bin_op),
            RegexExp::UnOp(un_op) => self.un_op(un_op),
        }
    }

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

    fn bin_op(&mut self, bin_op: &BinOp) -> NFA {
        let op = &bin_op.op;
        let nfa1 = self.build_from_regex(&bin_op.left);
        let nfa2 = self.build_from_regex(&bin_op.right);
        match op {
            &BinaryOperator::Concat => self.concat(&nfa1, &nfa2),
            &BinaryOperator::Union => self.union(&nfa1, &nfa2),
        }
    }

    fn un_op(&mut self, un_op: &UnOp) -> NFA {
        let op = &un_op.op;
        let nfa = self.build_from_regex(&un_op.operand);
        match op {
            &UnaryOperator::KleeneStar => self.kleene_star(&nfa),
            &UnaryOperator::Plus => self.one_or_more(&nfa),
            &UnaryOperator::Optional => self.optional(&nfa),
        }
    }

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

    fn kleene_star(&mut self, nfa: &NFA) -> NFA {
        let q0 = self.current_state;
        let qf = self.current_state + 1;
        self.current_state += 2;

        let mut transitions = HashMap::new();
        transitions.insert((q0, Symbol::Epsilon), HashSet::from([nfa.q0, qf]));
        transitions.insert((nfa.qf, Symbol::Epsilon), HashSet::from([nfa.q0, qf]));

        for ((q, c), p) in nfa.d.iter() {
            transitions.insert((*q, c.clone()), p.clone());
        }

        NFA {
            q0,
            qf,
            d: transitions,
        }
    }

    fn one_or_more(&mut self, nfa: &NFA) -> NFA {
        let nfa2 = self.kleene_star(nfa);
        return self.concat(nfa, &nfa2);
    }

    fn optional(&mut self, nfa: &NFA) -> NFA {
        let q0 = nfa.q0;
        let qf = nfa.qf;
        let mut d = nfa.d.clone();
        d.insert((q0, Symbol::Epsilon), HashSet::from([qf]));
        NFA { q0, qf, d }
    }
}
