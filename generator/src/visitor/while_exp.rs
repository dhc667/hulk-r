use super::{GeneratorVisitor, VisitorResult};

impl GeneratorVisitor {
    pub(crate) fn handle_while(
        &mut self,
        condition_result: VisitorResult,
        body_result: VisitorResult,
    ) -> VisitorResult {
        let condition_result_handle = condition_result
            .result_handle
            .expect("Expected a result handle for condition of while statement");

        let (loop_label, body_label, loop_exit_label) = self.generate_loop_labels();

        // here we assume the type of the handle returned by the condition is i1, SA is
        // responsible for this
        let loop_setup = self.branch_jump_statement(&loop_label)
            + &self.block_start(&loop_label)
            + &condition_result.preamble
            + &self.branch_choice_statement(
                &condition_result_handle.llvm_name,
                &body_label,
                &loop_exit_label,
            );

        let body_code = self.block_start(&body_label)
            + &body_result.preamble
            + &self.branch_jump_statement(&loop_label);

        let exit_code = self.block_start(&loop_exit_label);

        let preamble = loop_setup + &body_code + &exit_code;

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    /// # Description
    ///
    /// Uses the same global tmp_variable id to create globally unique loop, body,
    /// loop_exit labels
    ///
    /// # Examples
    ///
    /// - If we generate a temporary variable %.0, and then generate these labels, we'll get
    /// loop.1, body.1, loop_exit.1
    /// - If we generate the labels first, we'll get loop.0, body.0, loop_exit.0
    pub(crate) fn generate_loop_labels(&mut self) -> (String, String, String) {
        let l = format!("loop.{}", self.tmp_variable_id);
        let b = format!("body.{}", self.tmp_variable_id);
        let le = format!("loop_exit.{}", self.tmp_variable_id);

        self.tmp_variable_id += 1;

        (l, b, le)
    }
}
