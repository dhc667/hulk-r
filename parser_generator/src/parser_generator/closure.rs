use std::{collections::HashSet, fmt::Debug, hash::Hash};

use crate::{
    SymbolId,
    parser_generator::{
        ParserGenerator,
        item::{LR0Item, LR1Item, LR1ItemSetBuilder},
    },
    symbol::TerminalId,
};

impl<TokenType: Eq + Hash + Copy + Debug, R> ParserGenerator<TokenType, R> {
    // SetOfItems CLOSURE(I)
    pub(crate) fn lr1_closure(&self, i: HashSet<LR1Item>) -> HashSet<LR1Item> {
        let mut i = LR1ItemSetBuilder::from(i);

        loop {
            let mut to_add = LR1ItemSetBuilder::new();

            // for ( each item [A -> p.Bs, a] in I )
            for (core, follow) in i.items.iter() {
                let production_id = core.production_id;
                let production = self.productions.get(&production_id).unwrap();

                let b = self.symbol_right_of_dot(core);
                if b.is_none() || !b.unwrap().is_non_terminal_id() {
                    continue;
                }
                let b = *b.unwrap().as_non_terminal_id().unwrap();

                // for ( each terminal b in FIRST(sa) )
                let follow = self.first_of_symbols_with_appended_terminal_set(
                    production
                        .rhs
                        .iter()
                        .skip(core.dot_position + 1)
                        .into_iter(),
                    follow,
                );

                // for ( each production B -> y in G' )
                let b_productions = Self::productions_for_symbol(b, &self.productions);

                // add [B ->.y, b] to set I ;
                for (prod_id, _) in b_productions {
                    to_add
                        .insert(LR1Item::build(LR0Item::new(*prod_id, 0), follow.clone()).unwrap());
                }
            }

            // until no more items are added to I
            if !i.merge(to_add) {
                break;
            }
        }

        return i.to_hash_set();
    }

    // SetOfItems CLOSURE(I)
    pub(crate) fn lr0_closure(&self, i: HashSet<LR0Item>) -> HashSet<LR0Item> {
        let mut i = HashSet::from(i);

        loop {
            let mut to_add = HashSet::new();

            // for ( each item [A -> p.Bs] in I )
            for core in i.iter() {
                let b = self.symbol_right_of_dot(core);
                if b.is_none() || !b.unwrap().is_non_terminal_id() {
                    continue;
                }
                let b = *b.unwrap().as_non_terminal_id().unwrap();

                // for ( each production B -> y in G' )
                let b_productions = Self::productions_for_symbol(b, &self.productions);

                // add [B ->.y] to set I ;
                for (prod_id, _) in b_productions {
                    to_add.insert(LR0Item::new(*prod_id, 0));
                }
            }

            // until no more items are added to I
            let mut changed = false;
            for elem in to_add {
                changed |= i.insert(elem);
            }

            if !changed {
                break;
            }
        }

        return i;
    }

    fn first_of_symbols_with_appended_terminal_set<'a>(
        &'a self,
        mut symbols: impl Iterator<Item = &'a SymbolId>,
        terminals: &HashSet<TerminalId>,
    ) -> HashSet<TerminalId> {
        let first_symbol = symbols.next();

        // we check if there are no symbols
        let mut new_follow = vec![self.epsilon].into_iter().collect();
        if first_symbol != None {
            let symbols = vec![first_symbol.unwrap()].into_iter().chain(symbols);
            new_follow = self.compute_first(symbols);
        }

        if new_follow.contains(&self.epsilon) {
            new_follow.remove(&self.epsilon);
            terminals.iter().for_each(|terminal| {
                new_follow.insert(*terminal);
            });
        }

        new_follow
    }
}
