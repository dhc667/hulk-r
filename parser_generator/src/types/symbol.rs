use std::{fmt::Debug, hash::Hash};

use crate::Token;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SymbolId {
    TerminalId(TerminalId),
    NonTerminalId(NonTerminalId),
}

impl SymbolId {
    pub fn id_string(&self) -> String {
        match self {
            SymbolId::TerminalId(terminal_id) => terminal_id.id_string(),
            SymbolId::NonTerminalId(non_terminal_id) => non_terminal_id.id_string(),
        }
    }
}

impl From<NonTerminalId> for SymbolId {
    fn from(v: NonTerminalId) -> Self {
        Self::NonTerminalId(v)
    }
}

impl From<TerminalId> for SymbolId {
    fn from(v: TerminalId) -> Self {
        Self::TerminalId(v)
    }
}

pub type TerminalCompute<TokenType, R> = Box<dyn for<'a> Fn(&'a Token<TokenType>) -> R>;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct TerminalId(usize);

impl TerminalId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Eq, Hash, Debug, Clone, Copy, PartialEq)]
pub struct NonTerminalId(usize);

impl NonTerminalId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id_string(&self) -> String {
        self.0.to_string()
    }
}

impl SymbolId {
    pub fn as_non_terminal_id(&self) -> Option<&NonTerminalId> {
        if let Self::NonTerminalId(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the symbol id is [`NonTerminalId`].
    ///
    /// [`NonTerminalId`]: SymbolId::NonTerminalId
    #[must_use]
    pub fn is_non_terminal_id(&self) -> bool {
        matches!(self, Self::NonTerminalId(..))
    }

    /// Returns `true` if the symbol id is [`TerminalId`].
    ///
    /// [`TerminalId`]: SymbolId::TerminalId
    #[must_use]
    pub fn is_terminal_id(&self) -> bool {
        matches!(self, Self::TerminalId(..))
    }

    pub fn as_terminal_id(&self) -> Option<&TerminalId> {
        if let Self::TerminalId(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
