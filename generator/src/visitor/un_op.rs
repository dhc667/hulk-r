use ast::UnaryOperator;

use crate::llvm_types::{HandleType, LlvmHandle, LlvmType};

use super::{GeneratorVisitor, VisitorResult};

impl GeneratorVisitor {
    pub(crate) fn handle_un_op(
        &mut self,
        inner_result: VisitorResult,
        op: &UnaryOperator,
    ) -> VisitorResult {
        if inner_result.result_handle.is_none() {
            panic!("Expected a result handle for operand of unary operator");
        }

        match inner_result.result_handle.as_ref().unwrap().handle_type {
            HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                self.get_double_un_op_visitor_result(op, inner_result)
            }
            HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                self.get_boolean_un_op_visitor_result(op, inner_result)
            }
            HandleType::Literal(LlvmType::String) | HandleType::Register(LlvmType::String) => {
                self.get_string_un_op_visitor_result(op, inner_result)
            }
            HandleType::Literal(LlvmType::Object) | HandleType::Register(LlvmType::Object) => {
                self.get_object_un_op_visitor_result(op, inner_result)
            }
        }
    }

    fn get_double_un_op_visitor_result(
        &mut self,
        op: &ast::UnaryOperator,
        inner_result: VisitorResult,
    ) -> VisitorResult {
        let inner_handle = inner_result.result_handle.unwrap();

        match op {
            ast::UnaryOperator::Plus(_) => {
                return VisitorResult {
                    preamble: inner_result.preamble,
                    result_handle: Some(inner_handle),
                };
            }
            ast::UnaryOperator::Minus(_) => {
                let tmp_variable = self.generate_tmp_variable();
                let preamble = inner_result.preamble
                    + "\n"
                    + &format!(
                        "{} = fsub double 0.0, {}",
                        tmp_variable, inner_handle.llvm_name
                    );

                return VisitorResult {
                    preamble,
                    result_handle: Some(LlvmHandle::new_f64_register(tmp_variable)),
                };
            }
            _ => panic!("Unsupported unary operator for double"),
        }
    }

    fn get_boolean_un_op_visitor_result(
        &mut self,
        op: &ast::UnaryOperator,
        inner_result: VisitorResult,
    ) -> VisitorResult {
        let inner_handle = inner_result.result_handle.unwrap();

        match op {
            ast::UnaryOperator::Not(_) => {
                let tmp_variable = self.generate_tmp_variable();
                let preamble = inner_result.preamble
                    + "\n"
                    + &format!("{} = xor i1 {}, true", tmp_variable, inner_handle.llvm_name);

                return VisitorResult {
                    preamble,
                    result_handle: Some(LlvmHandle::new_i1_register(tmp_variable)),
                };
            }
            _ => panic!("Unsupported unary operator for boolean"),
        }
    }

    fn get_string_un_op_visitor_result(
        &mut self,
        op: &ast::UnaryOperator,
        inner_result: VisitorResult,
    ) -> VisitorResult {
        let inner_handle = inner_result.result_handle.unwrap();

        match op {
            _ => panic!("Unsupported unary operator for string"),
        }
    }

    fn get_object_un_op_visitor_result(
        &mut self,
        op: &ast::UnaryOperator,
        inner_result: VisitorResult,
    ) -> VisitorResult {
        let inner_handle = inner_result.result_handle.unwrap();

        match op {
            _ => panic!("Unsupported unary operator for object"),
        }
    }
}
