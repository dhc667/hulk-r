use std::hash::Hash;

use crate::symbol::{NonTerminal, NonTerminalId, Terminal, TerminalId};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SymbolId {
    TerminalId(TerminalId),
    NonTerminalId(NonTerminalId)
}

impl SymbolId {
    pub fn id_string(&self) -> String {
        match self {
            Self::TerminalId(id) => id.id_string(),
            Self::NonTerminalId(id) => id.id_string(),
        }
    }

    /// Returns `true` if the symbol id is [`TerminalId`].
    ///
    /// [`TerminalId`]: SymbolId::TerminalId
    #[must_use]
    pub fn is_terminal_id(&self) -> bool {
        matches!(self, Self::TerminalId(..))
    }

    /// Returns `true` if the symbol id is [`NonTerminalId`].
    ///
    /// [`NonTerminalId`]: SymbolId::NonTerminalId
    #[must_use]
    pub fn is_non_terminal_id(&self) -> bool {
        matches!(self, Self::NonTerminalId(..))
    }

    pub fn as_terminal_id(&self) -> Option<&TerminalId> {
        if let Self::TerminalId(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_non_terminal_id(&self) -> Option<&NonTerminalId> {
        if let Self::NonTerminalId(v) = self {
            Some(v)
        } else {
            None
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
