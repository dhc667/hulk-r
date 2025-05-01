mod assignment;
mod bin_op;
mod if_else;
mod print;
mod un_op;
mod while_exp;

mod helpers {
    pub mod control_flow;
    pub mod variables;
}

use std::collections::HashMap;

use crate::context::Context;
use crate::llvm_types::{LlvmHandle, LlvmType};
use ast::{Visitor, visitors::visitable::Visitable};

pub struct VisitorResult {
    pub result_handle: Option<LlvmHandle>,
    pub preamble: String,
}

impl VisitorResult {
    pub fn has_null_result(&self) -> bool {
        matches!(self.result_handle, None)
    }
}

struct Variable {
    var_type: LlvmType,
    llvm_name: String,
}

impl Variable {
    pub fn new_f64(llvm_name: String) -> Variable {
        Variable {
            var_type: LlvmType::F64,
            llvm_name,
        }
    }

    pub fn new_i1(llvm_name: String) -> Variable {
        Variable {
            var_type: LlvmType::I1,
            llvm_name,
        }
    }
}

pub struct GeneratorVisitor {
    /// # Description
    ///
    /// This will store the names of the llvm registers that store the
    /// pointers to the values of the variables defined in a given context
    ///
    /// ## Warning
    /// To define variables, use the define_or_shadow method of this class
    context: Context<Variable>,

    /// # Description
    ///
    /// Used to generate unique ids for temporary variables, irrespective
    /// of context, this way we don't need to worry about llvm's requirement
    /// that %N names be sequential starting at 0 within the same context
    tmp_variable_id: u32,

    /// # Description
    ///
    /// We need this in order to be able to shadow variables, or define
    /// variables with the same name in different contexts
    variable_ids: HashMap<String, i32>,
}

impl GeneratorVisitor {
    pub fn new() -> Self {
        GeneratorVisitor {
            context: Context::new_one_frame(),
            tmp_variable_id: 0,
            variable_ids: HashMap::new(),
        }
    }
}

impl Visitor<VisitorResult> for GeneratorVisitor {
    fn visit_program(&mut self, node: &mut ast::Program) -> VisitorResult {
        let mut program =
            self.instantiate_global_print_helpers() + "define i32 @main() {\nentry:\n";

        let inner = node.expression_list.accept(self);

        program = program + &inner.preamble;

        program = program + "\nret i32 0\n}\n";

        VisitorResult {
            preamble: program,
            result_handle: None,
        }
    }

    fn visit_expression_list(&mut self, node: &mut ast::ExpressionList) -> VisitorResult {
        let mut preamble = "".to_string();
        let mut result_handle = None;

        for exp in &mut node.expressions {
            let result = exp.accept(self);
            preamble = preamble + "\n" + &result.preamble;

            result_handle = result.result_handle;
        }

        VisitorResult {
            preamble,
            result_handle: if node.multiple_semicolon_terminated {
                None
            } else {
                result_handle
            },
        }
    }

    fn visit_expression(&mut self, node: &mut ast::Expression) -> VisitorResult {
        node.accept(self)
    }

    fn visit_destructive_assignment(
        &mut self,
        node: &mut ast::DestructiveAssignment,
    ) -> VisitorResult {
        let expression_result = node.expression.accept(self);
        let mut preamble = expression_result.preamble;

        let exp_result_handle = expression_result.result_handle.expect(
            "Variable must be dassigned to non-null expression result, SA should've caught this",
        );

        let var_llvm_name = &self
            .context
            .get_value(&node.identifier.id)
            .expect(
                format!(
                    "Variable {} not found, SA should have caught this",
                    node.identifier.id
                )
                .as_str(),
            )
            .llvm_name;

        preamble += &self.store_statement(
            &exp_result_handle.llvm_name,
            &var_llvm_name,
            &exp_result_handle.handle_type.inner_type(),
        );

        VisitorResult {
            preamble,
            result_handle: Some(exp_result_handle),
        }
    }

    fn visit_bin_op(&mut self, node: &mut ast::BinOp) -> VisitorResult {
        let lhs_result = node.lhs.accept(self);
        let rhs_result = node.rhs.accept(self);

        self.handle_bin_op(lhs_result, rhs_result, &node.op)
    }

    fn visit_atom(&mut self, node: &mut ast::Atom) -> VisitorResult {
        node.accept(self)
    }

    fn visit_let_in(&mut self, node: &mut ast::LetIn) -> VisitorResult {
        self.context.push_frame(true);

        let assignment_preamble = node.assignment.accept(self).preamble;

        let result = node.body.accept(self);

        self.context.pop_frame();

        VisitorResult {
            result_handle: result.result_handle,
            preamble: assignment_preamble + &result.preamble,
        }
    }

    fn visit_assignment(&mut self, node: &mut ast::Assignment) -> VisitorResult {
        let expression_result = node.rhs.accept(self);

        self.handle_assignment(node.identifier.id.clone(), expression_result)
    }

    fn visit_if_else(&mut self, node: &mut ast::IfElse) -> VisitorResult {
        let condition_result = node.condition.accept(self);

        let then_result = node.then_expression.accept(self);
        let else_result = node.else_expression.accept(self);

        self.handle_if_else(condition_result, then_result, else_result)
    }

    fn visit_print(&mut self, node: &mut ast::Print) -> VisitorResult {
        let inner_result = node.expression.accept(self);

        self.handle_print(inner_result)
    }

    fn visit_while(&mut self, node: &mut ast::While) -> VisitorResult {
        let condition_result = node.condition.accept(self);
        let body_result = node.body.accept(self);

        self.handle_while(condition_result, body_result)
    }

    fn visit_block(&mut self, node: &mut ast::Block) -> VisitorResult {
        self.context.push_frame(true);
        let result = node.expression_list.accept(self);
        self.context.pop_frame();

        result
    }

    fn visit_un_op(&mut self, node: &mut ast::UnOp) -> VisitorResult {
        let inner_result = node.rhs.accept(self);

        self.handle_un_op(inner_result, &node.op)
    }

    fn visit_variable(&mut self, node: &mut ast::Identifier) -> VisitorResult {
        let register_name = self.generate_tmp_variable();

        let variable = self
            .context
            .get_value(&node.id)
            .expect(format!("Variable {} not found, SA should have caught this", node.id).as_str());

        let (preamble, result_handle) = self.extract_variable_value_to_register(
            register_name,
            &variable.llvm_name,
            &variable.var_type,
        );

        return VisitorResult {
            preamble,
            result_handle: Some(result_handle),
        };
    }

    fn visit_number_literal(&mut self, node: &mut ast::NumberLiteral) -> VisitorResult {
        VisitorResult {
            preamble: "".to_string(),
            result_handle: Some(LlvmHandle::new_f64_literal(node.value)),
        }
    }

    fn visit_empty_expression(&mut self) -> VisitorResult {
        VisitorResult {
            preamble: "".to_string(),
            result_handle: None,
        }
    }

    fn visit_boolean_literal(&mut self, node: &mut ast::BooleanLiteral) -> VisitorResult {
        let bool_value = match node {
            ast::BooleanLiteral::True(_) => true,
            ast::BooleanLiteral::False(_) => false,
        };

        VisitorResult {
            preamble: String::new(),
            result_handle: Some(LlvmHandle::new_i1_literal(bool_value)),
        }
    }
}
