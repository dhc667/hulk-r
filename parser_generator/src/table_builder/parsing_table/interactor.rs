use crate::table_builder::{TableBuilder, conflicts::Conflict};

use super::LR0States;

impl<'b> TableBuilder<'b> {
    pub(crate) fn build_parsing_tables(&mut self) -> Result<(), Vec<Conflict>> {
        let LR0States {
            initial_state,
            initial_production_core,
            lr0_states,
            goto_table,
        } = self.build_lr0_states();

        let lr1_states = self.build_lr1_states(
            &initial_state,
            &initial_production_core,
            lr0_states,
            &goto_table,
        );

        // self.dbg_states(&lr1_states);

        let action_table = self.build_action_table(
            initial_state,
            initial_production_core,
            &lr1_states,
            &goto_table,
        )?;

        // self.dbg_action_table(&action_table);

        self.action_table = action_table;
        self.goto = goto_table;

        Ok(())
    }
}
