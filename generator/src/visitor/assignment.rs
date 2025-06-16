use crate::llvm_types::LlvmType;

use super::{GeneratorVisitor, Variable, VisitorResult};

impl GeneratorVisitor {
    pub(crate) fn handle_assignment(
        &mut self,
        identifier: String,
        expression_result: VisitorResult,
    ) -> VisitorResult {
        let mut preamble = expression_result.preamble;
        let result_handle = expression_result.result_handle.expect(
            "Variable must be assigned to non-null expression result, SA should've caught this",
        );

        let var_llvm_name =
            self.define_or_shadow(identifier, result_handle.handle_type.inner_type());

        preamble = preamble
            + &self.alloca_statement(&var_llvm_name, &result_handle.handle_type.inner_type())
            + &self.store_statement(
                &result_handle.llvm_name,
                &var_llvm_name,
                &result_handle.handle_type.inner_type(),
            );

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    /// # Description
    ///
    /// Increases the globally unique id for this variable name, defines it
    /// using its name in the current context, and assigning to it the unique
    /// generated llvm name
    ///
    /// Returns the newly generated llvm name
    ///
    /// # Examples
    ///
    /// The first generated name for a hulk variable `x` would for example be
    /// `%x.0`, the second, `%x.1`, this way, even if we enter and leave contexts,
    /// we do not need to have the concept of blocks in llvm
    pub(crate) fn define_or_shadow(&mut self, name: String, handle_type: LlvmType) -> String {
        let id: u32;
        {
            id = *self.variable_ids.get(&name).unwrap_or(&0);
        }
        self.variable_ids.insert(name.clone(), id + 1);

        let llvm_name = format!("%{}.{}", name, id);

        match handle_type {
            LlvmType::F64 => {
                self.context
                    .define(name, Variable::new_f64(llvm_name.clone()));
            }
            LlvmType::I1 => {
                self.context
                    .define(name, Variable::new_i1(llvm_name.clone()));
            }
            LlvmType::String => {
                self.context
                    .define(name, Variable::new_string(llvm_name.clone()));
            }
            LlvmType::Object => {
                self.context
                    .define(name, Variable::new_object(llvm_name.clone()));
            }
            LlvmType::List => {
                self.context
                    .define(name, Variable::new_list(llvm_name.clone()));
            }
        }

        return llvm_name;
    }
}
