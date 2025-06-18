use crate::llvm_types::{self, HandleType, LlvmType};
use std::str;

use super::{GeneratorVisitor, VisitorResult};

impl GeneratorVisitor {
    pub fn instantiate_global_print_helpers(&self) -> String {
        // Add string_type struct, constructor, and helper
        let _string_type_helpers = r#"
            %string_type = type { i8*, i64 }

            define void @string_type_constructor(%string_type* %this, i8* %data, i64 %len) {
            entry:
              ; Get pointer to the data field (first element of struct)
              %data_ptr = getelementptr inbounds %string_type, %string_type* %this, i32 0, i32 0
              ; Store the data pointer
              store i8* %data, i8** %data_ptr, align 8

              ; Get pointer to the length field (second element of struct)
              %len_ptr = getelementptr inbounds %string_type, %string_type* %this, i32 0, i32 1
              ; Store the length
              store i64 %len, i64* %len_ptr, align 8

              ret void
            }

            ; Helper function to create a string_type instance - FIXED to use malloc
            define %string_type* @create_string_type(i8* %data, i64 %len) {
            entry:
              ; Allocate memory on heap instead of stack (16 bytes for the struct)
              %str_obj_raw = call i8* @malloc(i64 16)
              %str_obj = bitcast i8* %str_obj_raw to %string_type*

              ; Call the constructor
              call void @string_type_constructor(%string_type* %str_obj, i8* %data, i64 %len)

              ret %string_type* %str_obj
            }

            ; Function to print a string_type (works with any string_type, not just globals)
            define void @print_string_type(%string_type* %str) {
            entry:
              ; Load the data pointer from the string_type
              %data_ptr = getelementptr inbounds %string_type, %string_type* %str, i32 0, i32 0
              %data = load i8*, i8** %data_ptr, align 8

              ; Load the length from the string_type
              %len_ptr = getelementptr inbounds %string_type, %string_type* %str, i32 0, i32 1
              %len = load i64, i64* %len_ptr, align 8

              ; Convert length to i32 for printf (printf expects int for precision)
              %len_i32 = trunc i64 %len to i32

              ; Get format string pointer
              %fmt_ptr = getelementptr inbounds [5 x i8], [5 x i8]* @.fmt, i64 0, i64 0

              ; Call printf with format "%.*s", length, and data
              call i32 @printf(i8* %fmt_ptr, i32 %len_i32, i8* %data)

              ret void
            }

        "#;

        "@.fstr = private constant [4 x i8] c\"%f\\0A\\00\", align 1\n".to_string()
            + "@.fstr2 = private constant [3 x i8] c\"%f\\00\", align 1\n"
            + "@.true_str = private constant [5 x i8] c\"true\\00\", align 1\n"
            + "@.false_str = private constant [6 x i8] c\"false\\00\", align 1\n"
            + "@.none_str = private constant [5 x i8] c\"none\\00\", align 1\n"
            + "@.space_str = private constant [2 x i8] c\" \\00\", align 1\n"
            + "declare i32 @printf(i8*, ...)\n"
            + "declare i32 @sprintf(i8*, i8*, ...)\n"
            + "declare i8* @strcat(i8*, i8*)\n"
            + "declare i8* @strcpy(i8*, i8*)\n"
            + "declare i32 @strlen(i8*)\n"
            + "declare i32 @strcmp(i8*, i8*)\n"
            + "declare i8* @malloc(i64)\n"
            + "@.fmt = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\", align 1"
            + "\n"
    }

    pub(crate) fn handle_print(&mut self, inner_result: VisitorResult) -> VisitorResult {
        let preamble = inner_result.preamble
            + &match inner_result.result_handle {
                Some(ref handle) => match handle.handle_type {
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
                    HandleType::Register(LlvmType::String)
                    | HandleType::Literal(LlvmType::String) => self.print_string(&handle.llvm_name),
                    HandleType::Register(LlvmType::Object)
                    | HandleType::Literal(LlvmType::Object) => self.print_object(&handle.llvm_name),
                    _ => panic!("Not handle"),
                },
                None => self.print_none(),
            };

        VisitorResult {
            preamble,
            result_handle: inner_result.result_handle,
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

    // FIXED: Changed parameter type from i8* to %string_type*
    fn print_string(&mut self, handle: &str) -> String {
        let tmp_var = self.generate_tmp_variable();
        format!(
            "{} = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0\n",
            tmp_var
        ) + &format!(
            "call i32 (i8*, ...) @printf(i8* {}, i8* {})\n",
            tmp_var, handle
        )
    }

    // FIXED: Changed parameter type from i8* to %string_type*
    fn print_object(&mut self, handle: &str) -> String {
        let tmp_var = self.generate_tmp_variable();
        format!(
            "{} = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0\n",
            tmp_var
        ) + &format!(
            "call i32 (i8*, ...) @printf(i8* {}, i8* {})\n",
            tmp_var, handle
        )
    }
}
