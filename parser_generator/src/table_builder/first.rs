use std::collections::{HashMap, HashSet};

use crate::{NonTerminalId, Production, SymbolId, TerminalId};

use crate::table_builder::TableBuilder;

impl<'b> TableBuilder<'b> {
    pub(crate) fn compute_first<'a>(
        &self,
        symbols: impl Iterator<Item = &'a SymbolId>,
    ) -> HashSet<TerminalId> {
        let mut answ = HashSet::new();
        let mut epsilon_found = true;

        for s in symbols {
            if !epsilon_found {
                break;
            } else {
                epsilon_found = false;
            }

            for f in self.first.get(&s).unwrap_or(&HashSet::new()) {
                if *f != self.epsilon {
                    answ.insert(*f);
                } else {
                    epsilon_found = true;
                }
            }
        }

        if epsilon_found {
            answ.insert(self.epsilon);
        }

        answ
    }

    pub(crate) fn compute_first_for_all_symbols(&mut self) {
        self.compute_terminal_firsts();
        self.compute_non_terminal_firsts();

        #[cfg(test)]
        self.dbg_first()
    }

    fn compute_terminal_firsts(&mut self) {
        for terminal_symbol in Self::terminal_symbols(&self.symbols) {
            let mut first = HashSet::new();
            first.insert(terminal_symbol);

            let symbol_id = SymbolId::from(terminal_symbol);
            self.first.insert(symbol_id, first);
        }
    }

    fn compute_non_terminal_firsts(&mut self) {
        let mut changed = true;

        while changed {
            changed = false;

            for non_terminal_symbol in Self::non_terminal_symbols(&self.symbols) {
                for (_, production) in
                    Self::productions_for_symbol(non_terminal_symbol, &self.productions)
                {
                    changed |= Self::update_first_with_production(
                        &mut self.first,
                        production,
                        self.epsilon,
                    );
                }
            }
        }
    }

    fn non_terminal_symbols(
        symbols: &HashMap<SymbolId, Option<String>>,
    ) -> impl Iterator<Item = NonTerminalId> {
        symbols
            .iter()
            .map(|(s, _)| *s)
            .filter(|s| s.is_non_terminal_id())
            .map(|s| *s.as_non_terminal_id().unwrap())
    }

    fn terminal_symbols(
        symbols: &HashMap<SymbolId, Option<String>>,
    ) -> impl Iterator<Item = TerminalId> {
        symbols
            .iter()
            .map(|(s, _)| *s)
            .filter(|s| s.is_terminal_id())
            .map(|s| *s.as_terminal_id().unwrap())
    }

    fn update_first_with_production(
        first: &mut HashMap<SymbolId, HashSet<TerminalId>>,
        production: &Production,
        epsilon: TerminalId,
    ) -> bool {
        let symbol_id = SymbolId::from(production.lhs);

        if !first.contains_key(&symbol_id) {
            first.insert(symbol_id, HashSet::new());
        }

        let mut new_non_terminal_first = first.remove(&symbol_id).unwrap();

        let mut changed = false;
        let mut epsilon_found = true;

        for symbol in production.rhs.iter() {
            if !epsilon_found {
                break;
            }

            epsilon_found = false;

            let symbol_first = first.get(&symbol);
            if symbol_first.is_none() {
                break;
            }
            let symbol_first = symbol_first.unwrap();

            for terminal in symbol_first {
                if *terminal == epsilon {
                    epsilon_found = true;
                } else {
                    changed |= new_non_terminal_first.insert(*terminal);
                }
            }
        }

        if epsilon_found {
            changed |= new_non_terminal_first.insert(epsilon);
        }

        first.insert(symbol_id, new_non_terminal_first);

        changed
    }

    //  NOTE: Useful for debugging
    //
    /// prints the first sets to standard error
    #[cfg(test)]
    fn dbg_first(&self) {
        let str = self
            .first
            .iter()
            .map(|(s_id, set)| {
                use crate::debugging_helpers::get_name_or_default;

                format!(
                    "{}: {}",
                    get_name_or_default(s_id, &self.symbols),
                    set.iter()
                        .map(|t| get_name_or_default(&SymbolId::from(*t), &self.symbols))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        eprintln!("{}", str);
    }
}
