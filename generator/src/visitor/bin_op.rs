use ast::BinaryOperator;
use ast::BinaryOperator::*;

use crate::llvm_types::{HandleType, LlvmHandle, LlvmType};

use super::{GeneratorVisitor, VisitorResult};

impl GeneratorVisitor {
    pub(crate) fn handle_bin_op(
        &mut self,
        lhs_result: VisitorResult,
        rhs_result: VisitorResult,
        op: &BinaryOperator,
    ) -> VisitorResult {
        if let At(_) | AtAt(_) = op {
            return self.get_string_bin_op_visitor_result(op, lhs_result, rhs_result);
        }

        if lhs_result.result_handle.is_none() {
            panic!("Expected a result handle for lhs of binary operator");
        }

        if rhs_result.result_handle.is_none() {
            panic!("Expected a result handle for rhs of binary operator");
        }

        // equal types for each operand is a guarantee of SA
        match lhs_result.result_handle.as_ref().unwrap().handle_type {
            HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                return self.get_double_bin_op_visitor_result(op, lhs_result, rhs_result);
            }
            HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                return self.get_boolean_bin_op_visitor_result(op, lhs_result, rhs_result);
            }
            HandleType::Literal(LlvmType::String) | HandleType::Register(LlvmType::String) => {
                return self.get_string_bin_op_visitor_result(op, lhs_result, rhs_result);
            }
            HandleType::Literal(LlvmType::Object) | HandleType::Register(LlvmType::Object) => {
                return self.get_object_bin_op_visitor_result(op, lhs_result, rhs_result);
            }
            _ => panic!("Unsupported operand type"),
        };
    }

    fn get_double_bin_op_visitor_result(
        &mut self,
        op: &ast::BinaryOperator,
        lhs: VisitorResult,
        rhs: VisitorResult,
    ) -> VisitorResult {
        let rhs_handle = rhs.result_handle.unwrap();
        let lhs_handle = lhs.result_handle.unwrap();

        let preamble = lhs.preamble + &rhs.preamble;

        let result_handle = self.generate_tmp_variable();

        let operation = match op {
            Plus(_) => format!(
                "{} = fadd double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            Minus(_) => format!(
                "{} = fsub double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            Times(_) => format!(
                "{} = fmul double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            Divide(_) => format!(
                "{} = fdiv double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            FloorDivide(_) => todo!(),
            Modulo(_) => todo!(),
            Equal(_) => panic!("= found in non-assignment, parser problem"),
            ColonEqual(_) => panic!(":= found in non-destructive assignment, parser problem"),
            EqualEqual(_) => format!(
                "{} = fcmp oeq double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            NotEqual(_) => format!(
                "{} = fcmp one double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            Less(_) => format!(
                "{} = fcmp olt double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            LessEqual(_) => format!(
                "{} = fcmp ole double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            Greater(_) => format!(
                "{} = fcmp ogt double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            GreaterEqual(_) => format!(
                "{} = fcmp oge double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            _ => panic!("Unsupported numeric operator"),
        } + "\n";

        let result_handle = match op {
            EqualEqual(_) | NotEqual(_) | Less(_) | LessEqual(_) | Greater(_) | GreaterEqual(_) => {
                Some(LlvmHandle::new_i1_register(result_handle))
            }

            Plus(_) | Minus(_) | Times(_) | Divide(_) | FloorDivide(_) | Modulo(_) => {
                Some(LlvmHandle::new_f64_register(result_handle))
            }

            Equal(_) => panic!("= found in non-assignment, parser problem"),
            ColonEqual(_) => panic!(":= found in non-destructive assignment, parser problem"),

            _ => panic!("Unsupported numeric operator"),
        };

        VisitorResult {
            preamble: preamble + &operation,
            result_handle,
        }
    }

    fn get_boolean_bin_op_visitor_result(
        &mut self,
        op: &ast::BinaryOperator,
        lhs: VisitorResult,
        rhs: VisitorResult,
    ) -> VisitorResult {
        let lhs_handle = lhs.result_handle.unwrap();
        let rhs_handle = rhs.result_handle.unwrap();
        let result_register = self.generate_tmp_variable();

        let operation = match op {
            And(_) => format!(
                "{} = and i1 {}, {}\n",
                result_register, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            Or(_) => format!(
                "{} = or i1 {}, {}\n",
                result_register, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            EqualEqual(_) => format!(
                "{} = icmp eq i1 {}, {}\n",
                result_register, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            NotEqual(_) => format!(
                "{} = icmp ne i1 {}, {}\n",
                result_register, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            _ => panic!("Unsupported boolean operator"),
        } + "\n";

        VisitorResult {
            preamble: lhs.preamble + &rhs.preamble + &operation,
            result_handle: Some(LlvmHandle::new_i1_register(result_register)),
        }
    }

    fn get_string_bin_op_visitor_result(
        &mut self,
        op: &ast::BinaryOperator,
        lhs: VisitorResult,
        rhs: VisitorResult,
    ) -> VisitorResult {
        let lhs_handle = lhs.result_handle.unwrap();
        let rhs_handle = rhs.result_handle.unwrap();

        let preamble = lhs.preamble + &rhs.preamble;
        let result_handle = self.generate_tmp_variable();

        let operation = match op {
            At(_) => {
                // String concatenation implementation with type coercion
                // Convert lhs operand to string if needed
                let (lhs_str, lhs_conv) = match lhs_handle.handle_type {
                    HandleType::Literal(LlvmType::String)
                    | HandleType::Register(LlvmType::String) => {
                        (lhs_handle.llvm_name.clone(), String::new())
                    }
                    HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                        let true_ptr = self.generate_tmp_variable();
                        let false_ptr = self.generate_tmp_variable();
                        let str_ptr = self.generate_tmp_variable();
                        let code = format!(
                            "{true_ptr} = getelementptr [6 x i8], [6 x i8]* @.true_str, i32 0, i32 0\n\
                             {false_ptr} = getelementptr [7 x i8], [7 x i8]* @.false_str, i32 0, i32 0\n\
                             {str_ptr} = select i1 {cond}, i8* {true_ptr}, i8* {false_ptr}\n",
                            true_ptr = true_ptr,
                            false_ptr = false_ptr,
                            str_ptr = str_ptr,
                            cond = lhs_handle.llvm_name
                        );
                        (str_ptr, code)
                    }
                    HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                        let num_buffer = self.generate_tmp_variable();
                        let buffer_cast = self.generate_tmp_variable();
                        let sn_ret = self.generate_tmp_variable();
                        let str_len = self.generate_tmp_variable();
                        let str_len_null = self.generate_tmp_variable();
                        let result_buffer = self.generate_tmp_variable();
                        let code = format!(
                            "{num_buffer} = alloca [32 x i8], align 1\n\
                             {buffer_cast} = bitcast [32 x i8]* {num_buffer} to i8*\n\
                             {sn_ret} = call i32 (i8*, i8*, ...) @sprintf(i8* {buffer_cast}, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fstr2, i32 0, i32 0), double {val})\n\
                             {str_len} = call i64 @strlen(i8* {buffer_cast})\n\
                             {str_len_null} = add i64 {str_len}, 1\n\
                             {result_buffer} = call i8* @malloc(i64 {str_len_null})\n\
                             call void @strcpy(i8* {result_buffer}, i8* {buffer_cast})\n",
                            num_buffer = num_buffer,
                            buffer_cast = buffer_cast,
                            sn_ret = sn_ret,
                            str_len = str_len,
                            str_len_null = str_len_null,
                            result_buffer = result_buffer,
                            val = lhs_handle.llvm_name
                        );
                        (result_buffer, code)
                    }
                    _ => panic!("Unsupported type for string concatenation"),
                };
                // Convert rhs operand to string if needed
                let (rhs_str, rhs_conv) = match rhs_handle.handle_type {
                    HandleType::Literal(LlvmType::String)
                    | HandleType::Register(LlvmType::String) => {
                        (rhs_handle.llvm_name.clone(), String::new())
                    }
                    HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                        let true_ptr = self.generate_tmp_variable();
                        let false_ptr = self.generate_tmp_variable();
                        let str_ptr = self.generate_tmp_variable();
                        let code = format!(
                            "{true_ptr} = getelementptr [6 x i8], [6 x i8]* @.true_str, i32 0, i32 0\n\
                             {false_ptr} = getelementptr [7 x i8], [7 x i8]* @.false_str, i32 0, i32 0\n\
                             {str_ptr} = select i1 {cond}, i8* {true_ptr}, i8* {false_ptr}\n",
                            true_ptr = true_ptr,
                            false_ptr = false_ptr,
                            str_ptr = str_ptr,
                            cond = rhs_handle.llvm_name
                        );
                        (str_ptr, code)
                    }
                    HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                        let num_buffer = self.generate_tmp_variable();
                        let buffer_cast = self.generate_tmp_variable();
                        let sn_ret = self.generate_tmp_variable();
                        let str_len = self.generate_tmp_variable();
                        let str_len_null = self.generate_tmp_variable();
                        let result_buffer = self.generate_tmp_variable();
                        let code = format!(
                            "{num_buffer} = alloca [32 x i8], align 1\n\
                             {buffer_cast} = bitcast [32 x i8]* {num_buffer} to i8*\n\
                             {sn_ret} = call i32 (i8*, i8*, ...) @sprintf(i8* {buffer_cast}, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fstr2, i32 0, i32 0), double {val})\n\
                             {str_len} = call i64 @strlen(i8* {buffer_cast})\n\
                             {str_len_null} = add i64 {str_len}, 1\n\
                             {result_buffer} = call i8* @malloc(i64 {str_len_null})\n\
                             call void @strcpy(i8* {result_buffer}, i8* {buffer_cast})\n",
                            num_buffer = num_buffer,
                            buffer_cast = buffer_cast,
                            sn_ret = sn_ret,
                            str_len = str_len,
                            str_len_null = str_len_null,
                            result_buffer = result_buffer,
                            val = rhs_handle.llvm_name
                        );
                        (result_buffer, code)
                    }
                    _ => panic!("Unsupported type for string concatenation"),
                };
                // Combine preambles for conversions
                let len1 = self.generate_tmp_variable();
                let len2 = self.generate_tmp_variable();
                let total = self.generate_tmp_variable();
                let total_plus_one = self.generate_tmp_variable();
                let result_ptr = self.generate_tmp_variable();
                lhs_conv
                    + &rhs_conv
                    + &format!(
                        // Get lengths and allocate + concatenate as before
                        "{len1} = call i32 @strlen(i8* {lhs})\n\
                     {len2} = call i32 @strlen(i8* {rhs})\n\
                     {total} = add i32 {len1}, {len2}\n\
                     {total_plus_one} = add i32 {total}, 1\n\
                     {result_ptr} = call i8* @malloc(i32 {total_plus_one})\n\
                     call i8* @strcpy(i8* {result_ptr}, i8* {lhs})\n\
                     {res} = call i8* @strcat(i8* {result_ptr}, i8* {rhs})\n",
                        len1 = len1,
                        len2 = len2,
                        total = total,
                        total_plus_one = total_plus_one,
                        result_ptr = result_ptr,
                        lhs = lhs_str,
                        rhs = rhs_str,
                        res = result_handle
                    )
            }

            AtAt(_) => {
                // String concatenation implementation with type coercion
                // Convert lhs operand to string if needed
                let (lhs_str, lhs_conv) = match lhs_handle.handle_type {
                    HandleType::Literal(LlvmType::String)
                    | HandleType::Register(LlvmType::String) => {
                        (lhs_handle.llvm_name.clone(), String::new())
                    }
                    HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                        let true_ptr = self.generate_tmp_variable();
                        let false_ptr = self.generate_tmp_variable();
                        let str_ptr = self.generate_tmp_variable();
                        let code = format!(
                            "{true_ptr} = getelementptr [6 x i8], [6 x i8]* @.true_str, i32 0, i32 0\n\
                             {false_ptr} = getelementptr [7 x i8], [7 x i8]* @.false_str, i32 0, i32 0\n\
                             {str_ptr} = select i1 {cond}, i8* {true_ptr}, i8* {false_ptr}\n",
                            true_ptr = true_ptr,
                            false_ptr = false_ptr,
                            str_ptr = str_ptr,
                            cond = lhs_handle.llvm_name
                        );
                        (str_ptr, code)
                    }
                    HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                        let num_buffer = self.generate_tmp_variable();
                        let buffer_cast = self.generate_tmp_variable();
                        let sn_ret = self.generate_tmp_variable();
                        let str_len = self.generate_tmp_variable();
                        let str_len_null = self.generate_tmp_variable();
                        let result_buffer = self.generate_tmp_variable();
                        let code = format!(
                            "{num_buffer} = alloca [32 x i8], align 1\n\
                             {buffer_cast} = bitcast [32 x i8]* {num_buffer} to i8*\n\
                             {sn_ret} = call i32 (i8*, i8*, ...) @sprintf(i8* {buffer_cast}, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fstr2, i32 0, i32 0), double {val})\n\
                             {str_len} = call i64 @strlen(i8* {buffer_cast})\n\
                             {str_len_null} = add i64 {str_len}, 1\n\
                             {result_buffer} = call i8* @malloc(i64 {str_len_null})\n\
                             call void @strcpy(i8* {result_buffer}, i8* {buffer_cast})\n",
                            num_buffer = num_buffer,
                            buffer_cast = buffer_cast,
                            sn_ret = sn_ret,
                            str_len = str_len,
                            str_len_null = str_len_null,
                            result_buffer = result_buffer,
                            val = lhs_handle.llvm_name
                        );
                        (result_buffer, code)
                    }
                    _ => panic!("Unsupported type for string concatenation"),
                };
                // Convert rhs operand to string if needed
                let (rhs_str, rhs_conv) = match rhs_handle.handle_type {
                    HandleType::Literal(LlvmType::String)
                    | HandleType::Register(LlvmType::String) => {
                        (rhs_handle.llvm_name.clone(), String::new())
                    }
                    HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                        let true_ptr = self.generate_tmp_variable();
                        let false_ptr = self.generate_tmp_variable();
                        let str_ptr = self.generate_tmp_variable();
                        let code = format!(
                            "{true_ptr} = getelementptr [6 x i8], [6 x i8]* @.true_str, i32 0, i32 0\n\
                             {false_ptr} = getelementptr [7 x i8], [7 x i8]* @.false_str, i32 0, i32 0\n\
                             {str_ptr} = select i1 {cond}, i8* {true_ptr}, i8* {false_ptr}\n",
                            true_ptr = true_ptr,
                            false_ptr = false_ptr,
                            str_ptr = str_ptr,
                            cond = rhs_handle.llvm_name
                        );
                        (str_ptr, code)
                    }
                    HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                        let num_buffer = self.generate_tmp_variable();
                        let buffer_cast = self.generate_tmp_variable();
                        let sn_ret = self.generate_tmp_variable();
                        let str_len = self.generate_tmp_variable();
                        let str_len_null = self.generate_tmp_variable();
                        let result_buffer = self.generate_tmp_variable();
                        let code = format!(
                            "{num_buffer} = alloca [32 x i8], align 1\n\
                             {buffer_cast} = bitcast [32 x i8]* {num_buffer} to i8*\n\
                             {sn_ret} = call i32 (i8*, i8*, ...) @sprintf(i8* {buffer_cast}, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fstr2, i32 0, i32 0), double {val})\n\
                             {str_len} = call i64 @strlen(i8* {buffer_cast})\n\
                             {str_len_null} = add i64 {str_len}, 1\n\
                             {result_buffer} = call i8* @malloc(i64 {str_len_null})\n\
                             call void @strcpy(i8* {result_buffer}, i8* {buffer_cast})\n",
                            num_buffer = num_buffer,
                            buffer_cast = buffer_cast,
                            sn_ret = sn_ret,
                            str_len = str_len,
                            str_len_null = str_len_null,
                            result_buffer = result_buffer,
                            val = rhs_handle.llvm_name
                        );
                        (result_buffer, code)
                    }
                    _ => panic!("Unsupported type for string concatenation"),
                };
                // Combine preambles for conversions
                let len1 = self.generate_tmp_variable();
                let len2 = self.generate_tmp_variable();
                let total = self.generate_tmp_variable();
                let total_plus_one = self.generate_tmp_variable();
                let result_ptr = self.generate_tmp_variable();
                let space_ptr = self.generate_tmp_variable();
                lhs_conv
                    + &rhs_conv
                    + &format!(
                        // Get lengths and allocate + concatenate with space in the middle
                        "{len1} = call i32 @strlen(i8* {lhs})\n\
                     {len2} = call i32 @strlen(i8* {rhs})\n\
                     {total} = add i32 {len1}, {len2}\n\
                     {total_plus_one} = add i32 {total}, 1\n\
                     {result_ptr} = call i8* @malloc(i32 {total_plus_one})\n\
                     call i8* @strcpy(i8* {result_ptr}, i8* {lhs})\n\
                     {space_ptr} = getelementptr [2 x i8], [2 x i8]* @.space_str, i32 0, i32 0\n\
                     call i8* @strcat(i8* {result_ptr}, i8* {space_ptr})\n\
                     {res} = call i8* @strcat(i8* {result_ptr}, i8* {rhs})\n",
                        len1 = len1,
                        len2 = len2,
                        total = total,
                        total_plus_one = total_plus_one,
                        result_ptr = result_ptr,
                        lhs = lhs_str,
                        space_ptr = space_ptr,
                        res = result_handle,
                        rhs = rhs_str
                    )
            }

            EqualEqual(_) => format!(
                "{} = fcmp oeq double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            _ => panic!("Unsupported string operator"),
        } + "\n";

        let result_handle = match op {
            EqualEqual(_) | NotEqual(_) | Less(_) | LessEqual(_) | Greater(_) | GreaterEqual(_) => {
                Some(LlvmHandle::new_i1_register(result_handle))
            }
            Plus(_) => Some(LlvmHandle::new_string_register(result_handle)),
            At(_) | AtAt(_) => Some(LlvmHandle::new_string_register(result_handle)),
            Equal(_) => panic!("= found in non-assignment, parser problem"),
            ColonEqual(_) => panic!(":= found in non-destructive assignment, parser problem"),
            _ => panic!("Unsupported string operator"),
        };

        VisitorResult {
            preamble: preamble + &operation,
            result_handle,
        }
    }

    fn get_object_bin_op_visitor_result(
        &mut self,
        op: &ast::BinaryOperator,
        lhs: VisitorResult,
        rhs: VisitorResult,
    ) -> VisitorResult {
        let lhs_handle = lhs.result_handle.unwrap();
        let rhs_handle = rhs.result_handle.unwrap();

        let preamble = lhs.preamble + &rhs.preamble;
        let result_handle = self.generate_tmp_variable();

        let operation = match op {
            EqualEqual(_) => format!(
                "{} = fcmp oeq double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            _ => panic!("Unsupported object operator"),
        } + "\n";

        let result_handle = match op {
            EqualEqual(_) | NotEqual(_) => Some(LlvmHandle::new_i1_register(result_handle)),
            Equal(_) => panic!("= found in non-assignment, parser problem"),
            ColonEqual(_) => panic!(":= found in non-destructive assignment, parser problem"),
            _ => panic!("Unsupported object operator"),
        };

        VisitorResult {
            preamble: preamble + &operation,
            result_handle,
        }
    }
}
