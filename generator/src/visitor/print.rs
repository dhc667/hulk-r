use crate::llvm_types::{self, HandleType, LlvmType};

use super::{GeneratorVisitor, VisitorResult};

impl GeneratorVisitor {
    pub fn instantiate_global_print_helpers(&self) -> String {
        "@.fstr = private constant [4 x i8] c\"%f\\0A\\00\", align 1\n".to_string()
            + "@.true_str = private constant [6 x i8] c\"true\\0A\\00\", align 1\n"
            + "@.false_str = private constant [7 x i8] c\"false\\0A\\00\", align 1\n"
            + "@.none_str = private constant [6 x i8] c\"none\\0A\\00\", align 1\n"
            + "declare i32 @printf(i8*, ...)\n"
    }

    pub(crate) fn handle_print(&mut self, inner_result: VisitorResult) -> VisitorResult {
        let preamble = inner_result.preamble
            + &match inner_result.result_handle {
                Some(handle) => match handle.handle_type {
                    HandleType::Register(LlvmType::F64) | HandleType::Literal(LlvmType::F64) => {
                        self.print_double(&handle.llvm_name)
                    }
                    HandleType::Literal(LlvmType::I1) => {
                        if handle.llvm_name == llvm_types::TRUE_LITERAL_STR {
                            self.print_true()
                        } else {
                            self.print_false()
                        }
                    }
                    HandleType::Register(LlvmType::I1) => {
                        self.print_boolean_register(&handle.llvm_name)
                    }
                },
                None => self.print_none(),
            };

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    fn print_true(&mut self) -> String {
        let element_ptr = self.generate_tmp_variable();
        format!(
            "{} = getelementptr [6 x i8], [6 x i8]* @.true_str, i32 0, i32 0\n",
            element_ptr
        ) + &format!("call i32 (i8*, ...) @printf(i8* {})\n", element_ptr)
    }

    fn print_false(&mut self) -> String {
        let element_ptr = self.generate_tmp_variable();
        format!(
            "{} = getelementptr [7 x i8], [7 x i8]* @.false_str, i32 0, i32 0\n",
            element_ptr
        ) + &format!("call i32 (i8*, ...) @printf(i8* {})\n", element_ptr)
    }

    fn print_boolean_register(&mut self, handle: &str) -> String {
        let (then, else_, fi) = self.generate_then_else_fi_labels();
        format!("br i1 {handle}, label %{then}, label %{else_}\n")
            + &format!("{then}:\n")
            + &self.print_true()
            + &format!("br label %{fi}\n")
            + &format!("{else_}:\n")
            + &self.print_false()
            + &format!("br label %{fi}\n")
            + &format!("{fi}:\n")
    }

    fn print_double(&mut self, handle: &str) -> String {
        let element_ptr_variable = self.generate_tmp_variable();

        format!(
            "{} = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0\ncall i32 (i8*, ...) @printf(i8* {}, double {})\n",
            element_ptr_variable, element_ptr_variable, handle
        )
    }

    fn print_none(&mut self) -> String {
        let element_ptr = self.generate_tmp_variable();
        format!(
            "{} = getelementptr [6 x i8], [6 x i8]* @.none_str, i32 0, i32 0\n",
            element_ptr
        ) + &format!("call i32 (i8*, ...) @printf(i8* {})\n", element_ptr)
    }
}
