use std::collections::HashMap;

use parser::BinaryOperator::*;
use parser::{Visitor, visitors::visitable::Visitable};

use crate::context::Context;
use crate::llvm_types::{HandleType, LlvmHandle, LlvmType};


pub struct VisitorResult {
    pub result_handle: Option<LlvmHandle>,
    pub preamble: String,
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

    pub fn generate_new_tmp_variable(&mut self) -> String {
        // we use the . after % to get around llvm's requirement that %N names
        // be sequential starting at 0 within the same context
        let tmp_variable = format!("%.{}", self.tmp_variable_id);
        self.tmp_variable_id += 1;

        tmp_variable
    }

    /// # Description
    ///
    /// Increases the globally unique id for this variable name, defines it
    /// using its name in the current context, and assigning to it the unique
    /// generated llvm name
    ///
    /// Returns the newly generated llvm name
    ///
    /// # Example
    ///
    /// The first generated name for a hulk variable `x` would for example be
    /// `%x.0`, the second, `%x.1`, this way, even if we enter and leave contexts,
    /// we do not need to have the concept of blocks in llvm
    pub fn define_or_shadow(&mut self, name: String) -> String {
        let id: i32;
        {
            id = *self.variable_ids.get(&name).unwrap_or(&0);
        }
        self.variable_ids.insert(name.clone(), id + 1);

        let llvm_name = format!("%{}.{}", name, id);

        self.context
            .define(name, Variable::new_f64(llvm_name.clone()));

        return llvm_name;
    }
}

impl Visitor<VisitorResult> for GeneratorVisitor {
    fn visit_program(&mut self, node: &mut parser::Program) -> VisitorResult {
        let mut program = "@.fstr = private constant [2 x i8] c\"%f\", align 1\n".to_string()
            + "declare i32 @printf(i8*, ...)\n"
            + "define i32 @main() {\nentry:\n";

        let inner = node.expression_list.accept(self);

        program = program + &inner.preamble;

        program = program + "\nret i32 0\n}\n";

        VisitorResult {
            preamble: program,
            result_handle: None,
        }
    }

    fn visit_expression_list(&mut self, node: &mut parser::ExpressionList) -> VisitorResult {
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

    fn visit_expression(&mut self, node: &mut parser::Expression) -> VisitorResult {
        node.accept(self)
    }

    fn visit_destructive_assignment(
        &mut self,
        node: &mut parser::DestructiveAssignment,
    ) -> VisitorResult {
        let expression_result = node.expression.accept(self);
        let mut preamble = expression_result.preamble;

        let result_handle = expression_result.result_handle.expect(
            "Variable must be dassigned to non-null expression result, SA should've caught this",
        );

        let llvm_name = &self.context
            .get_value(&node.identifier.id)
            .expect(format!("Variable {} not found, SA should have caught this", node.identifier.id).as_str())
            .llvm_name;

        match result_handle.handle_type {
            HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                preamble = preamble
                    + &format!(
                        "store double {}, double* {}, align 4\n",
                        result_handle.handle, llvm_name
                    )
            }
        };

        VisitorResult {
            preamble,
            result_handle: Some(result_handle),
        }
    }

    fn visit_bin_op(&mut self, node: &mut parser::BinOp) -> VisitorResult {
        let left_result = node.lhs.accept(self);
        let left_handle = left_result
            .result_handle
            .expect("Expected a result handle for lhs of binary operator");
        let right_result = node.rhs.accept(self);
        let right_handle = right_result
            .result_handle
            .expect("Expected a result handle for rhs of binary operator");

        let preamble = left_result.preamble + &right_result.preamble;

        let result_handle = self.generate_new_tmp_variable();

        let operation = match node.op {
            Plus(_) => format!(
                "{} = fadd double {}, {}",
                result_handle, left_handle.handle, right_handle.handle
            ),
            Minus(_) => format!(
                "{} = fsub double {}, {}",
                result_handle, left_handle.handle, right_handle.handle
            ),
            Times(_) => format!(
                "{} = fmul double {}, {}",
                result_handle, left_handle.handle, right_handle.handle
            ),
            Divide(_) => format!(
                "{} = fdiv double {}, {}",
                result_handle, left_handle.handle, right_handle.handle
            ),
            //  TODO: these will need some setup
            FloorDivide(_) => todo!(),
            Modulo(_) => todo!(),
            Equal(_) => todo!(),

            ColonEqual(_) => panic!(":= found in non-destructive assignment, parser problem"),

            //  TODO: these are not even implemented in the parser
            EqualEqual(_) => todo!(),
            Less(_) => todo!(),
            LessEqual(_) => todo!(),
            Greater(_) => todo!(),
            GreaterEqual(_) => todo!(),
        } + "\n";

        VisitorResult {
            preamble: preamble + &operation,
            result_handle: Some(LlvmHandle::new_tmp_register(result_handle)),
        }
    }

    fn visit_atom(&mut self, node: &mut parser::Atom) -> VisitorResult {
        node.accept(self)
    }

    fn visit_let_in(&mut self, node: &mut parser::LetIn) -> VisitorResult {
        self.context.push_frame(true);

        let assignment_preamble = node.assignment.accept(self).preamble;

        let result = node.body.accept(self);

        self.context.pop_frame();

        VisitorResult {
            result_handle: result.result_handle,
            preamble: assignment_preamble + &result.preamble,
        }
    }

    fn visit_assignment(&mut self, node: &mut parser::Assignment) -> VisitorResult {
        let expression_result = node.rhs.accept(self);
        let mut preamble = expression_result.preamble;
        let result_handle = expression_result.result_handle.expect(
            "Variable must be assigned to non-null expression result, SA should've caught this",
        );

        let llvm_name = self.define_or_shadow(node.identifier.id.clone());

        match result_handle.handle_type {
            HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                preamble = preamble
                    + &format!("{} = alloca double, align 4\n", llvm_name)
                    + &format!(
                        "store double {}, double* {}, align 4\n",
                        result_handle.handle, llvm_name
                    )
            }
        };

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    fn visit_if_else(&mut self, node: &mut parser::IfElse) -> VisitorResult {
        todo!()
    }

    fn visit_print(&mut self, node: &mut parser::Print) -> VisitorResult {
        let inner_result = node.expression.accept(self);
        let element_ptr_variable = self.generate_new_tmp_variable();

        let preamble = inner_result.preamble
            + &format!(
                "{} = getelementptr inbounds [2 x i8], [2 x i8]* @.fstr, i32 0, i32 0\ncall i32 (i8*, ...) @printf(i8* {}, double {})",
                element_ptr_variable,
                element_ptr_variable,
                inner_result
                    .result_handle
                    .expect("Expected a result handle for operand of unary operator")
                    .handle
            );

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    fn visit_while(&mut self, node: &mut parser::While) -> VisitorResult {
        todo!()
    }

    fn visit_block(&mut self, node: &mut parser::Block) -> VisitorResult {
        self.context.push_frame(true);
        let result = node.expression_list.accept(self);
        self.context.pop_frame();

        result
    }

    fn visit_un_op(&mut self, node: &mut parser::UnOp) -> VisitorResult {
        let inner_result = node.rhs.accept(self);

        match node.op {
            parser::UnaryOperator::Plus(_) => VisitorResult {
                preamble: inner_result.preamble,
                result_handle: inner_result.result_handle,
            },
            parser::UnaryOperator::Minus(_) => {
                let tmp_variable = self.generate_new_tmp_variable();
                let preamble = inner_result.preamble
                    + "\n"
                    + &format!(
                        "{} = fsub double 0.0, {}",
                        tmp_variable,
                        inner_result
                            .result_handle
                            .expect("Expected a result handle for operand of unary operator")
                            .handle
                    );

                VisitorResult {
                    preamble,
                    result_handle: Some(LlvmHandle::new_tmp_register(tmp_variable)),
                }
            }
        }
    }

    fn visit_variable(&mut self, node: &mut parser::Identifier) -> VisitorResult {
        let register_name = self.generate_new_tmp_variable();

        let variable = self
            .context
            .get_value(&node.id)
            .expect(format!("Variable {} not found, SA should have caught this", node.id).as_str());

        match variable.var_type {
            LlvmType::F64 => {
                let preamble = format!(
                    "{} = load double, double* {}, align 4\n",
                    register_name, variable.llvm_name
                );

                return VisitorResult {
                    preamble,
                    result_handle: Some(LlvmHandle::new_tmp_register(register_name)),
                };
            }
        }
    }

    fn visit_number_literal(&mut self, node: &mut parser::NumberLiteral) -> VisitorResult {
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
}
