use crate::llvm_types::LlvmHandle;

use super::{GeneratorVisitor, VisitorResult};

impl GeneratorVisitor {
    pub(crate) fn handle_if_else(
        &mut self,
        condition_result: VisitorResult,
        then_result: VisitorResult,
        else_result: VisitorResult,
    ) -> VisitorResult {
        let condition_preamble = condition_result.preamble;
        let condition_result_handle_name = condition_result
            .result_handle
            .expect("Expected result handle for condition of if expression")
            .llvm_name;

        match (then_result.result_handle, else_result.result_handle) {
            (Some(then_result_handle), Some(else_result_handle)) => self.handle_returning_if_else(
                &condition_preamble,
                &condition_result_handle_name,
                &then_result.preamble,
                &then_result_handle,
                &else_result.preamble,
                &else_result_handle,
            ),
            (None, None) => self.handle_none_returning_if_else(
                condition_preamble,
                &condition_result_handle_name,
                &then_result.preamble,
                &else_result.preamble,
            ),
            _ => panic!(
                "Detected if expression with different return types, SA should have caught this"
            ),
        }
    }

    fn handle_none_returning_if_else(
        &mut self,
        condition_preamble: String,
        condition_result_handle_name: &str,
        then_preamble: &str,
        else_preamble: &str,
    ) -> VisitorResult {
        let (then_label, else_label, fi_label) = self.generate_then_else_fi_labels();

        let condition_setup = condition_preamble
            + &self.branch_choice_statement(condition_result_handle_name, &then_label, &else_label);

        let then_branch =
            self.block_start(&then_label) + then_preamble + &self.branch_jump_statement(&fi_label);

        let else_branch =
            self.block_start(&else_label) + else_preamble + &self.branch_jump_statement(&fi_label);

        let final_block = self.block_start(&fi_label);

        let preamble = condition_setup + &then_branch + &else_branch + &final_block;

        return VisitorResult {
            result_handle: None,
            preamble,
        };
    }

    fn handle_returning_if_else(
        &mut self,
        condition_preamble: &str,
        condition_result_handle_name: &str,
        then_preamble: &str,
        then_result_handle: &LlvmHandle,
        else_preamble: &str,
        else_result_handle: &LlvmHandle,
    ) -> VisitorResult {
        let (then_label, else_label, fi_label) = self.generate_then_else_fi_labels();

        let result_var_register = self.generate_tmp_variable();
        let result_type = then_result_handle.handle_type.inner_type();

        // we create a variable to store the result (to avoid phi)
        let condition_setup = self.alloca_statement(&result_var_register, &result_type)
            + condition_preamble
            + &self.branch_choice_statement(condition_result_handle_name, &then_label, &else_label);

        // we create both branches, they start with their respective labels, then their preamble,
        // and finally they store the result in the result variable

        let then_branch = self.block_start(&then_label)
            + then_preamble
            + &self.store_statement(
                &then_result_handle.llvm_name,
                &result_var_register,
                &result_type,
            )
            + &self.branch_jump_statement(&fi_label);

        let else_branch = self.block_start(&else_label)
            + else_preamble
            + &self.store_statement(
                &else_result_handle.llvm_name,
                &result_var_register,
                &result_type,
            )
            + &self.branch_jump_statement(&fi_label);

        // finally, we extract the result into a new register in the final block

        let result_register = self.generate_tmp_variable();

        let (result_load_preamble, result_handle) = self.extract_variable_value_to_register(
            result_register,
            &result_var_register,
            &result_type,
        );

        let final_block = self.block_start(&fi_label) + &result_load_preamble;

        let preamble = condition_setup + &then_branch + &else_branch + &final_block;

        return VisitorResult {
            result_handle: Some(result_handle),
            preamble,
        };
    }

    /// # Description
    ///
    /// Uses the same global tmp_variable id to create globally unique then, else, fi
    /// labels, used for if expressions
    ///
    /// # Examples
    ///
    /// - If we generate a temporary variable %.0, and then generate these labels, we'll get
    /// then.1, else.1, fi.1
    /// - If we generate the labels first, we'll get then.0, else.0, fi.0
    pub(crate) fn generate_then_else_fi_labels(&mut self) -> (String, String, String) {
        let t = format!("then.{}", self.tmp_variable_id);
        let e = format!("else.{}", self.tmp_variable_id);
        let f = format!("fi.{}", self.tmp_variable_id);

        self.tmp_variable_id += 1;

        (t, e, f)
    }
}
