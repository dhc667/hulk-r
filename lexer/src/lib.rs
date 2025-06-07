use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

pub use grammar::RegexParser;

pub mod regex_engine;

#[cfg(test)]
pub mod test {
    pub mod char_matching;
    pub mod dfa_matching;
    pub mod nfa_matching;
    pub mod regex_parsing;
}
