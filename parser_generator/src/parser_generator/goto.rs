use std::{collections::HashSet, fmt::Debug, hash::Hash};

use crate::{
    SymbolId,
    parser_generator::{
        ParserGenerator,
        item::{LR0Item, LR1Item},
    },
};
impl<TokenType: Eq + Hash + Copy + Debug, R> ParserGenerator<TokenType, R> {
    // SetOfItems GOTO(I, X) {
    pub(crate) fn lr1_goto(&self, i: HashSet<LR1Item>, x: SymbolId) -> HashSet<LR1Item> {
        // initialize J to be the empty set;
        let mut answ = HashSet::new();

        // for ( each item [A -> p.Xs, a] in I )
        for item in i.iter().filter(|item| {
            let dot_position = item.core.dot_position;
            let production = self.productions.get(&item.core.production_id).unwrap();
            if dot_position >= production.rhs.len() {
                return false;
            }

            production.rhs[dot_position] == x
        }) {
            // add item [A -> pX.s, a] to set J ;
            let new_core = LR0Item::new(item.core.production_id, item.core.dot_position + 1);
            answ.insert(LR1Item::build(new_core, item.follow.clone()).unwrap());
        }

        // return CLOSURE(J);
        self.lr1_closure(answ)
    }

    // SetOfItems GOTO(I, X) {
    pub(crate) fn lr0_goto(&self, i: HashSet<LR0Item>, x: SymbolId) -> HashSet<LR0Item> {
        // initialize J to be the empty set;
        let mut answ = HashSet::new();

        // for ( each item [A -> p.Xs] in I )
        for core in i.iter().filter(|core| {
            let dot_position = core.dot_position;
            let production = self.productions.get(&core.production_id).unwrap();
            if dot_position >= production.rhs.len() {
                return false;
            }

            production.rhs[dot_position] == x
        }) {
            // add item [A -> pX.s] to set J ;
            answ.insert(LR0Item::new(core.production_id, core.dot_position + 1));
        }

        // return CLOSURE(J);
        self.lr0_closure(answ)
    }
}
