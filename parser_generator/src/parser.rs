mod action;
pub use action::Action;

mod parser_input;
use parser_input::Parse;

mod parse_error;
pub use parse_error::ParseError;

use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::{
    Production, Token,
    parser_generator::get_name_or_default,
    symbol::{SymbolId, Terminal, TerminalId},
};

#[derive(Debug)]
pub struct Parser<TokenType: Eq + Hash, R> {
    action_table: HashMap<(StateId, TerminalId), Action>,
    productions: HashMap<ProductionId, Production<R>>,
    terminals: HashMap<TerminalId, Terminal<TokenType, R>>,
    goto: HashMap<(StateId, SymbolId), StateId>,
    symbol_table: HashMap<TokenType, TerminalId>,
    symbol_names: HashMap<SymbolId, Option<String>>,
    eof_id: TerminalId,
}

impl<'a, TokenType: Eq + Hash + Copy + Debug, R> Parser<TokenType, R> {
    pub(crate) fn new(
        parsing_table: HashMap<(StateId, TerminalId), Action>,
        productions: HashMap<ProductionId, Production<R>>,
        terminals: HashMap<TerminalId, Terminal<TokenType, R>>,
        symbol_table: HashMap<TokenType, TerminalId>,
        symbol_names: HashMap<SymbolId, Option<String>>,
        goto: HashMap<(StateId, SymbolId), StateId>,
        eof_id: TerminalId,
    ) -> Self {
        Self {
            action_table: parsing_table,
            productions,
            terminals,
            symbol_table,
            symbol_names,
            goto,
            eof_id,
        }
    }

    pub fn parse(&self, tokens: Vec<Token<TokenType>>) -> Result<R, ParseError<TokenType>> {
        let mut current_parse = Parse::new(tokens);

        loop {
            let next_terminal_id = self.get_next_token_terminal_id(&current_parse);

            let current_state = self.get_current_state(&current_parse);
            let action = self.get_next_action(current_state, next_terminal_id);

            if let None = action {
                let next_token = self.get_next_token(&current_parse);

                return match next_token {
                    Some(tok) => Err(ParseError::UnexpectedToken {
                        ty: tok.ty,
                        loc: tok.start,
                    }),
                    None => Err(ParseError::UnexpectedEof),
                };
            }

            let action = action.unwrap();

            match action {
                Action::Shift(state_id) => self.shift(*state_id, &mut current_parse),
                Action::Reduce(production_id) => self.reduce(*production_id, &mut current_parse),
                Action::Accept => {
                    break;
                }
            };
        }

        let answ = current_parse.value_stack.pop().unwrap();

        Ok(answ)
    }

    fn shift(&self, state_id: StateId, current_parse: &mut Parse<TokenType, R>) {
        let terminal = self.get_next_token_terminal(&current_parse);
        let token = self
            .get_next_token(&current_parse)
            .expect("Should not perform shift on end of file");

        let value = (*terminal.compute)(token);

        current_parse.state_stack.push(state_id);
        current_parse.value_stack.push(value);
        current_parse.token_index += 1;

        // eprintln!("Shift {}", state_id.0)
    }

    fn reduce(&self, production_id: ProductionId, current_parse: &mut Parse<TokenType, R>) {
        let production = self
            .productions
            .get(&production_id)
            .expect("production id in parse table should be valid");

        let prod_lhs_symbol_id = SymbolId::from(production.lhs);

        let symbol_count = production.rhs.len();
        let mut reduced_values = Vec::new();

        for _ in 0..symbol_count {
            reduced_values.push(current_parse.value_stack.pop().unwrap());
            current_parse.state_stack.pop().unwrap();
        }

        let current_state = *current_parse.state_stack.last().unwrap();

        reduced_values = reduced_values.into_iter().rev().collect();

        let new_value = (*production.compute)(reduced_values);

        current_parse.value_stack.push(new_value);

        // eprintln!("Reduce");

        let new_state = self.goto.get(&(current_state, prod_lhs_symbol_id)).unwrap();

        // eprintln!(
        //     "Goto {} by {}",
        //     new_state.0,
        //     get_name_or_default(&prod_lhs_symbol_id, &self.symbol_names)
        // );

        current_parse.state_stack.push(*new_state);
    }

    fn get_next_action(&self, current_state: StateId, next_symbol: TerminalId) -> Option<&Action> {
        let action = self.action_table.get(&(current_state, next_symbol));

        action
    }

    fn get_next_token_terminal_id(&self, current_parse: &Parse<TokenType, R>) -> TerminalId {
        let token = self.get_next_token(current_parse);

        if token.is_none() {
            return self.eof_id;
        }

        let token = token.unwrap();

        let current_token_type = &token.ty;

        *self.symbol_table.get(current_token_type).unwrap()
    }

    fn get_next_token_terminal(
        &self,
        current_parse: &Parse<TokenType, R>,
    ) -> &Terminal<TokenType, R> {
        let terminal_id = self.get_next_token_terminal_id(current_parse);

        self.terminals.get(&terminal_id).unwrap()
    }

    fn get_next_token<'b>(
        &self,
        current_parse: &'b Parse<TokenType, R>,
    ) -> Option<&'b Token<TokenType>> {
        let tokens = &current_parse.tokens;
        let token_index = current_parse.token_index;

        if token_index >= tokens.len() {
            return None;
        }

        Some(&tokens[token_index])
    }

    fn get_current_state(&self, current_parse: &Parse<TokenType, R>) -> StateId {
        *current_parse.state_stack.last().unwrap()
    }


    fn get_valid_tokens_for_state(&self, state: StateId) -> Vec<&TokenType> {
        self.action_table
            .iter()
            .filter(|(key, _)| key.0 == state)
            .map(|(key, _)| {
                &self
                    .get_terminal(key.1)
                    .expect("terminal id in action table should be defined")
                    .token_type
            })
            .collect()
    }

    fn get_terminal(&self, terminal_id: TerminalId) -> Option<&Terminal<TokenType, R>> {
        self.terminals.get(&terminal_id)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
pub struct ProductionId(pub usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StateId(pub usize);
