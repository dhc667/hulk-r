use crate::RegexParser;
use crate::regex_engine::automata::dfa::DFA;
use crate::regex_engine::automata::nfa::NFA;
use crate::regex_engine::automata::nfa_builder::NFABuilder;

pub struct DFAMatcher {
    dfa: DFA,
}

impl DFAMatcher {
    pub fn new(regex: &str) -> Self {
        let parser = RegexParser::new();
        let regex_ast = parser.parse(regex).expect("Failed to parse regex");
        let mut builder = NFABuilder::new();
        let nfa = builder.build_from_regex(&regex_ast);
        let dfa = DFA::from(nfa);
        DFAMatcher { dfa }
    }

    pub fn matches(&self, input: &str) -> bool {
        self.dfa.simulate(input.chars().collect())
    }
}

pub struct NFAMatcher {
    nfa: NFA,
}

impl NFAMatcher {
    pub fn new(regex: &str) -> Self {
        let parser = RegexParser::new();
        let regex_ast = parser.parse(regex).expect("Failed to parse regex");
        let mut builder = NFABuilder::new();
        let nfa = builder.build_from_regex(&regex_ast);
        NFAMatcher { nfa }
    }

    pub fn matches(&self, input: &str) -> bool {
        self.nfa.simulate(input.chars().collect())
    }
}
