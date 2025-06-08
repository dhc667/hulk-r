use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

pub use grammar::RegexParser;

pub mod regex_engine;

pub mod lexer_generator;

pub mod automata_utils;

#[cfg(test)]
pub mod test {
    pub mod char_matching;
    pub mod dfa_matching;
    pub mod nfa_matching;
    pub mod regex_matching;
    pub mod regex_parsing;
}
