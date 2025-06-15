use std::{collections::HashMap, fmt::Debug, hash::Hash};

use error_handler::error::error::HulkError;
use error_handler::error::sintactic::user_error::UserError;

#[cfg(test)]
use crate::debugging_helpers::get_name_or_default;

use crate::grammar::Lex;
use crate::parser::{Action, Parse};
use crate::{
    ParseError, Production, ProductionCompute, ProductionId, StateId, SymbolId, TerminalCompute,
};
use crate::{TerminalId, Token};

pub struct Parser<TokenType: Eq + Hash + Debug + Copy, R> {
    action_table: HashMap<(StateId, TerminalId), Action>,
    symbols: HashMap<SymbolId, Option<String>>,
    productions: HashMap<ProductionId, Production>,
    production_computes: HashMap<ProductionId, ProductionCompute<R>>,
    terminal_computes: HashMap<TerminalId, TerminalCompute<TokenType, R>>,
    goto: HashMap<(StateId, SymbolId), StateId>,
    token_to_terminal: HashMap<TokenType, TerminalId>,

    eof_id: TerminalId,
}

impl<'a, TokenType: Eq + Hash + Copy + Debug, R> Parser<TokenType, R> {
    pub(crate) fn new(
        action_table: HashMap<(StateId, TerminalId), Action>,
        symbols: HashMap<SymbolId, Option<String>>,
        production_computes: HashMap<ProductionId, ProductionCompute<R>>,
        productions: HashMap<ProductionId, Production>,
        terminal_computes: HashMap<TerminalId, TerminalCompute<TokenType, R>>,
        token_to_terminal: HashMap<TokenType, TerminalId>,
        goto: HashMap<(StateId, SymbolId), StateId>,
        eof_id: TerminalId,
    ) -> Self {
        Self {
            action_table,
            symbols,
            production_computes,
            productions,
            terminal_computes,
            token_to_terminal,
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

    pub fn parse_with_lexer(
        &self,
        lexer: impl Lex<TokenType>,
        input: &str,
    ) -> Result<R, Vec<HulkError>> {
        let toks = lexer.split(input)?;

        self.parse(toks).map_err(|err| {
            let position = match err {
                ParseError::UnexpectedToken { loc, .. } => loc,
                ParseError::UnexpectedEof => input.len(),
            };
            vec![UserError::new(err.to_string(input), position).into()]
        })
    }

    fn shift(&self, state_id: StateId, current_parse: &mut Parse<TokenType, R>) {
        let compute = self.get_next_token_terminal_compute(&current_parse);
        let token = self
            .get_next_token(&current_parse)
            .expect("Should not perform shift on end of file");

        let value = compute(token);

        current_parse.state_stack.push(state_id);
        current_parse.value_stack.push(value);
        current_parse.token_index += 1;
    }

    fn reduce(&self, production_id: ProductionId, current_parse: &mut Parse<TokenType, R>) {
        let production = self
            .productions
            .get(&production_id)
            .expect("production id in parse table should have a production associated to it");

        let compute = self
            .production_computes
            .get(&production_id)
            .expect("production id in parse table should have a production compute function associated to it");

        let symbol_count = production.rhs.len();
        let mut reduced_values = Vec::new();
        for _ in 0..symbol_count {
            reduced_values.push(current_parse.value_stack.pop().unwrap());
            current_parse.state_stack.pop().unwrap();
        }
        reduced_values = reduced_values.into_iter().rev().collect();

        let new_value = compute(reduced_values);
        current_parse.value_stack.push(new_value);

        let lhs = SymbolId::from(production.lhs);
        let current_state = *current_parse.state_stack.last().unwrap();
        let new_state = self.goto.get(&(current_state, lhs)).unwrap();

        current_parse.state_stack.push(*new_state);

        #[cfg(test)]
        eprintln!(
            "Parsed {} -> {}",
            get_name_or_default(&SymbolId::from(production.lhs), &self.symbols),
            production
                .rhs
                .iter()
                .map(|s| get_name_or_default(s, &self.symbols))
                .collect::<Vec<_>>()
                .join(" ")
        )
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

        *self.token_to_terminal.get(current_token_type).unwrap()
    }

    fn get_next_token_terminal_compute(
        &self,
        current_parse: &Parse<TokenType, R>,
    ) -> &TerminalCompute<TokenType, R> {
        let terminal_id = self.get_next_token_terminal_id(current_parse);

        self.terminal_computes.get(&terminal_id).unwrap()
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

    #[cfg(test)]
    #[allow(dead_code)]
    fn get_valid_tokens_for_state(&self, state: StateId) -> Vec<&TokenType> {
        self.action_table
            .iter()
            .filter(|((s, _), _)| *s == state)
            .map(|((_, terminal), _)| self.get_token_type(terminal))
            .collect()
    }

    #[cfg(test)]
    #[allow(dead_code)]
    fn get_token_type(&self, terminal_id: &TerminalId) -> &TokenType {
        self.token_to_terminal
            .iter()
            .filter(|(_, terminal_id_i)| *terminal_id_i == terminal_id)
            .next()
            .unwrap()
            .0
    }
}

impl<TokenType: Eq + Hash + Debug + Copy, R> Debug for Parser<TokenType, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parser")
            .field("action_table", &self.action_table)
            .field("productions", &self.productions)
            .field(
                "production_computes",
                &self
                    .production_computes
                    .iter()
                    .map(|(id, _)| (id, "{ production compute function }"))
                    .collect::<HashMap<_, _>>(),
            )
            .field(
                "terminal_computes",
                &self
                    .terminal_computes
                    .iter()
                    .map(|(id, _)| (id, "{ terminal compute function }"))
                    .collect::<HashMap<_, _>>(),
            )
            .field("goto", &self.goto)
            .field("symbol_table", &self.token_to_terminal)
            .field("eof_id", &self.eof_id)
            .finish()
    }
}
