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
            Concat(_) => format!(
                "{} = call i8* @strcat(i8* {}, i8* {})",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            EqualEqual(_) => format!(
                "{} = fcmp oeq double {}, {}",
                result_handle, lhs_handle.llvm_name, rhs_handle.llvm_name
            ),
            _ => panic!("Unsupported string operator"),
        } + "\n";

        let result_handle = match op {
            EqualEqual(_) | NotEqual(_) | Less(_) | LessEqual(_) | Greater(_) | GreaterEqual(_) => {
                Some(LlvmHandle::new_i1_register(result_handle))
            },
            Plus(_) => {
                Some(LlvmHandle::new_string_register(result_handle))
            },
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
            EqualEqual(_) | NotEqual(_) => {
                Some(LlvmHandle::new_i1_register(result_handle))
            },
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
