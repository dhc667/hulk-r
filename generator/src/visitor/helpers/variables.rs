use crate::{
    GeneratorVisitor,
    llvm_types::{LlvmHandle, LlvmType},
};

impl GeneratorVisitor {
    pub fn generate_tmp_variable(&self) -> String {
        let current = self.tmp_counter.get();
        self.tmp_counter.set(current + 1);

        format!("%tmp{}", current)
    }

    pub(crate) fn alloca_statement(&self, var_register_name: &str, var_type: &LlvmType) -> String {
        let (type_name, align_size) = self.type_name_and_align_size(var_type);

        format!("{var_register_name} = alloca {type_name}, align {align_size}\n")
    }

    pub(crate) fn load_statement(
        &self,
        source_var_register_name: &str,
        target_register_name: &str,
        var_type: &LlvmType,
    ) -> String {
        let (type_name, align_size) = self.type_name_and_align_size(var_type);

        format!(
            "{target_register_name} = load {type_name}, {type_name}* {source_var_register_name}, align {align_size}\n"
        )
    }

    pub(crate) fn extract_variable_value_to_register(
        &self,
        target_register_name: String,
        source_var_register_name: &str,
        var_type: &LlvmType,
    ) -> (String, LlvmHandle) {
        let preamble =
            self.load_statement(source_var_register_name, &target_register_name, var_type);

        let result_handle = match var_type {
            LlvmType::F64 => LlvmHandle::new_f64_register(target_register_name),
            LlvmType::I1 => LlvmHandle::new_i1_register(target_register_name),
            LlvmType::String => LlvmHandle::new_string_register(target_register_name),
            LlvmType::Object => LlvmHandle::new_object_register(target_register_name),
            LlvmType::List(inner) => LlvmHandle::new_list_register(*inner.clone(), target_register_name),
        };

        (preamble, result_handle)
    }

    pub(crate) fn store_statement(
        &self,
        source_register_name: &str,
        target_var_register_name: &str,
        var_type: &LlvmType,
    ) -> String {
        let (type_name, align_size) = self.type_name_and_align_size(var_type);

        format!(
            "store {type_name} {source_register_name}, {type_name}* {target_var_register_name}, align {align_size}\n"
        )
    }

    pub(crate) fn type_name_and_align_size(&self, llvm_type: &LlvmType) -> (String, u32) {
        match llvm_type {
            LlvmType::F64 => ("double".to_string(), 8),
            LlvmType::I1 => ("i1".to_string(), 1),
            LlvmType::String => ("i8*".to_string(), 8),
            LlvmType::Object => ("i8*".to_string(), 8),
            LlvmType::List(inner) => (format!("{}*", inner.llvm_type_str()), 8),
        }
    }
}