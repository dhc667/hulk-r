use crate::GeneratorVisitor;

impl GeneratorVisitor {
    /// # Description
    ///
    /// Will format input as follows:
    ///
    /// format!("br i1 {condition_handle}, label %{true_label_name}, label %{false_label_name}")
    pub(crate) fn branch_choice_statement(
        &self,
        condition_handle: &str,
        true_label_name: &str,
        false_label_name: &str,
    ) -> String {
        format!("br i1 {condition_handle}, label %{true_label_name}, label %{false_label_name}\n")
    }

    /// # Description
    ///
    /// Will format input as follows:
    ///
    /// format!("br label %{label_name}")
    pub(crate) fn branch_jump_statement(&self, label_name: &str) -> String {
        format!("br label %{label_name}\n")
    }

    /// # Description
    ///
    /// Will format input as follows:
    ///
    /// format!("{label_name}:\n")
    pub(crate) fn block_start(&self, label_name: &str) -> String {
        format!("{label_name}:\n")
    }
}
