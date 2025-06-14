#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NonTerminalId(pub usize);

impl NonTerminalId {
    pub fn id_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Eq, Hash)]
pub struct NonTerminal {
    pub id: NonTerminalId,
}

impl NonTerminal {
    pub fn new(id: usize) -> Self {
        Self {
            id: NonTerminalId(id)
        }
    }

}

impl PartialEq for NonTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
