use std::collections::HashMap;

use crate::context::Context;
use crate::llvm_types;
use crate::llvm_types::{HandleType, LlvmHandle, LlvmType};
use ast::BinaryOperator::*;
use ast::{Visitor, visitors::visitable::Visitable};

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

    pub fn generate_tmp_variable(&mut self) -> String {
        // we use the . after % to get around llvm's requirement that %N names
        // be sequential starting at 0 within the same context
        let tmp_variable = format!("%.{}", self.tmp_variable_id);
        self.tmp_variable_id += 1;

        tmp_variable
    }

    /// # Description
    ///
    /// Uses the same global tmp_variable id to create globally unique then, else, fi
    /// labels, used for if expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use generator::GeneratorVisitor;
    /// let mut cg = GeneratorVisitor::new();
    ///
    /// let (t, e, f) = cg.generate_then_else_fi_labels();
    ///
    /// assert_eq!(t, "then.0");
    /// assert_eq!(e, "else.0");
    /// assert_eq!(f, "fi.0");
    /// ```
    ///
    /// ```rust
    /// use generator::GeneratorVisitor;
    /// let mut cg = GeneratorVisitor::new();
    ///
    /// let a = cg.generate_tmp_variable(); // %.0
    /// let (t, e, f) = cg.generate_then_else_fi_labels();
    ///
    /// assert_eq!(t, "then.1");
    /// assert_eq!(e, "else.1");
    /// assert_eq!(f, "fi.1");
    /// ```
    ///
    pub fn generate_then_else_fi_labels(&mut self) -> (String, String, String) {
        let t = format!("then.{}", self.tmp_variable_id);
        let e = format!("else.{}", self.tmp_variable_id);
        let f = format!("fi.{}", self.tmp_variable_id);

        self.tmp_variable_id += 1;

        (t, e, f)
    }

    /// # Description
    ///
    /// Uses the same global tmp_variable id to create globally unique loop, body,
    /// loop_exit labels
    ///
    /// # Examples
    ///
    /// ```rust
    /// use generator::GeneratorVisitor;
    /// let mut cg = GeneratorVisitor::new();
    ///
    /// let (l, b, le) = cg.generate_loop_labels();
    ///
    /// assert_eq!(l, "loop.0");
    /// assert_eq!(b, "body.0");
    /// assert_eq!(le, "loop_exit.0");
    /// ```
    ///
    /// ```rust
    /// use generator::GeneratorVisitor;
    /// let mut cg = GeneratorVisitor::new();
    ///
    /// let a = cg.generate_tmp_variable(); // %.0
    /// let (l, b, le) = cg.generate_loop_labels();
    ///
    /// assert_eq!(l, "loop.1");
    /// assert_eq!(b, "body.1");
    /// assert_eq!(le, "loop_exit.1");
    /// ```
    ///
    pub fn generate_loop_labels(&mut self) -> (String, String, String) {
        let l = format!("loop.{}", self.tmp_variable_id);
        let b = format!("body.{}", self.tmp_variable_id);
        let le = format!("loop_exit.{}", self.tmp_variable_id);

        self.tmp_variable_id += 1;

        (l, b, le)
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
    pub fn define_or_shadow(&mut self, name: String, handle_type: LlvmType) -> String {
        let id: i32;
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
        }

        return llvm_name;
    }

    /// # Description
    ///
    /// This will be used internally to create a visitor result when the
    /// lhs of a unary operator is double, to not fill the visit_unop
    /// function with too much code.
    ///
    /// It is assumed if the lhs is double then the rhs will also be double,
    /// this is a guarantee of SA
    ///
    /// # Panics
    ///
    /// - If the inner result handle is None or if the
    /// operator is not supported by double values
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
                    result_handle: Some(LlvmHandle::new_tmp_f64_register(tmp_variable)),
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
                    result_handle: Some(LlvmHandle::new_tmp_i1_register(tmp_variable)),
                };
            }
            _ => panic!("Unsupported unary operator for boolean"),
        }
    }

    /// # Description
    ///
    /// This will be used internally to create a visitor result when the
    /// operands of a binary operator are doubles, to not fill the
    /// visit_bin_op handler with code
    ///
    /// # Panics
    ///
    /// - If eiher of the operand handles are None or the operator is
    /// not supported for double values
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
                Some(LlvmHandle::new_tmp_i1_register(result_handle))
            }

            Plus(_) | Minus(_) | Times(_) | Divide(_) | FloorDivide(_) | Modulo(_) => {
                Some(LlvmHandle::new_tmp_f64_register(result_handle))
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
            result_handle: Some(LlvmHandle::new_tmp_i1_register(result_register)),
        }
    }

    fn print_true(&mut self) -> String {
        let element_ptr = self.generate_tmp_variable();
        format!(
            "{} = getelementptr [5 x i8], [5 x i8]* @.true_str, i32 0, i32 0\n",
            element_ptr
        ) + &format!("call i32 (i8*, ...) @printf(i8* {})\n", element_ptr)
    }

    fn print_false(&mut self) -> String {
        let element_ptr = self.generate_tmp_variable();
        format!(
            "{} = getelementptr [6 x i8], [6 x i8]* @.false_str, i32 0, i32 0\n",
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

    fn print_none(&mut self) -> String {
        let element_ptr = self.generate_tmp_variable();
        format!(
            "{} = getelementptr [5 x i8], [5 x i8]* @.none_str, i32 0, i32 0\n",
            element_ptr
        ) + &format!("call i32 (i8*, ...) @printf(i8* {})\n", element_ptr)
    }
}

impl Visitor<VisitorResult> for GeneratorVisitor {
    fn visit_program(&mut self, node: &mut ast::Program) -> VisitorResult {
        let mut program = "@.fstr = private constant [3 x i8] c\"%f\\00\", align 1\n".to_string()
            + "@.true_str = private constant [5 x i8] c\"true\\00\", align 1\n"
            + "@.false_str = private constant [6 x i8] c\"false\\00\", align 1\n"
            + "@.none_str = private constant [5 x i8] c\"none\\00\", align 1\n"
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

        let result_handle = expression_result.result_handle.expect(
            "Variable must be dassigned to non-null expression result, SA should've caught this",
        );

        let llvm_name = &self
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

        match result_handle.handle_type {
            HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                preamble = preamble
                    + &format!(
                        "store double {}, double* {}, align 8\n",
                        result_handle.llvm_name, llvm_name
                    )
            }
            HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                preamble = preamble
                    + &format!(
                        "store i1 {}, i1* {}, align 1\n",
                        result_handle.llvm_name, llvm_name
                    );
            }
        };

        VisitorResult {
            preamble,
            result_handle: Some(result_handle),
        }
    }

    fn visit_bin_op(&mut self, node: &mut ast::BinOp) -> VisitorResult {
        let left_result = node.lhs.accept(self);
        if left_result.result_handle.is_none() {
            panic!("Expected a result handle for lhs of binary operator");
        }

        let right_result = node.rhs.accept(self);
        if right_result.result_handle.is_none() {
            panic!("Expected a result handle for rhs of binary operator");
        }

        // equal types for each operand is a guarantee of SA
        match left_result.result_handle.as_ref().unwrap().handle_type {
            HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                return self.get_double_bin_op_visitor_result(&node.op, left_result, right_result);
            }
            HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                return self.get_boolean_bin_op_visitor_result(&node.op, left_result, right_result);
            }
        };
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
        let mut preamble = expression_result.preamble;
        let result_handle = expression_result.result_handle.expect(
            "Variable must be assigned to non-null expression result, SA should've caught this",
        );

        let llvm_name = self.define_or_shadow(
            node.identifier.id.clone(),
            result_handle.handle_type.inner_type(),
        );

        match result_handle.handle_type {
            HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                preamble = preamble
                    + &format!("{} = alloca double, align 8\n", llvm_name)
                    + &format!(
                        "store double {}, double* {}, align 8\n",
                        result_handle.llvm_name, llvm_name
                    )
            }
            HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                preamble = preamble
                    + &format!("{} = alloca i1, align 1\n", llvm_name)
                    + &format!(
                        "store i1 {}, i1* {}, align 1\n",
                        result_handle.llvm_name, llvm_name
                    );
            }
        };

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    fn visit_if_else(&mut self, node: &mut ast::IfElse) -> VisitorResult {
        let (then_label, else_label, fi_label) = self.generate_then_else_fi_labels();

        let condition_result = node.condition.accept(self);
        let condition_handle = condition_result
            .result_handle
            .expect("Expected a result handle for condition of if expression");

        let then_result = node.then_expression.accept(self);
        let else_result = node.else_expression.accept(self);

        let (result_variable, result_register, result_type) = match then_result.result_handle {
            Some(ref handle) => (
                Some(self.generate_tmp_variable()),
                Some(self.generate_tmp_variable()),
                Some(handle.handle_type),
            ),

            // this can happen if the then block is empty, or is multiple semicolon
            // terminated, we also assume the else block is empty in this case, SA
            // must guarantee this
            None => (None, None, None),
        };

        let format_result_store =
            |branch_result_handle: Option<LlvmHandle>| match (branch_result_handle, &result_type) {
                (
                    Some(ref name),
                    Some(HandleType::Literal(LlvmType::F64))
                    | Some(HandleType::Register(LlvmType::F64)),
                ) => format!(
                    "store double {}, double* {}, align 8\n",
                    name.llvm_name,
                    result_variable.as_ref().unwrap()
                ),
                (
                    Some(ref name),
                    Some(HandleType::Literal(LlvmType::I1))
                    | Some(HandleType::Register(LlvmType::I1)),
                ) => format!(
                    "store i1 {}, i1* {}, align 1\n",
                    name.llvm_name,
                    result_variable.as_ref().unwrap()
                ),
                _ => "".to_string(),
            };

        let result_alloca_statement = match (&result_variable, &result_type) {
            (
                Some(name),
                Some(HandleType::Literal(LlvmType::F64))
                | Some(HandleType::Register(LlvmType::F64)),
            ) => {
                format!("{} = alloca double, align 8\n", name)
            }
            (
                Some(name),
                Some(HandleType::Literal(LlvmType::I1)) | Some(HandleType::Register(LlvmType::I1)),
            ) => {
                format!("{} = alloca i1, align 1\n", name)
            }
            _ => "".to_string(),
        };

        let result_load_statement = match (&result_register, &result_variable, &result_type) {
            (
                Some(reg),
                Some(var),
                Some(HandleType::Literal(LlvmType::F64))
                | Some(HandleType::Register(LlvmType::F64)),
            ) => {
                format!("{} = load double, double* {}, align 8\n", reg, var)
            }
            (
                Some(reg),
                Some(var),
                Some(HandleType::Literal(LlvmType::I1)) | Some(HandleType::Register(LlvmType::I1)),
            ) => {
                format!("{} = load i1, i1* {}, align 1\n", reg, var)
            }
            _ => "".to_string(),
        };

        let mut branch_setup = condition_result.preamble;

        match condition_handle.handle_type {
            HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                branch_setup = branch_setup
                    + &result_alloca_statement
                    + &format!(
                        "br i1 {}, label %{}, label %{}\n",
                        condition_handle.llvm_name, then_label, else_label
                    );
            }
            _ => panic!("Expected a boolean handle for condition of if expression"),
        };

        let format_branch = |branch_name, preamble, result_handle: Option<LlvmHandle>| {
            format!(
                "{}:\n{}",
                branch_name,
                preamble
                    + format_result_store(result_handle).as_str()
                    + format!("br label %{}\n", fi_label).as_str()
            )
        };

        let then_code = format_branch(then_label, then_result.preamble, then_result.result_handle);
        let else_code = format_branch(else_label, else_result.preamble, else_result.result_handle);

        let preamble = branch_setup
            + &then_code
            + &else_code
            + &format!("{}:\n", fi_label)
            + &result_load_statement;

        VisitorResult {
            preamble,
            result_handle: result_register.zip(result_type).map(|(name, ht)| match ht {
                HandleType::Register(LlvmType::F64) | HandleType::Literal(LlvmType::F64) => {
                    LlvmHandle::new_tmp_f64_register(name)
                }
                HandleType::Register(LlvmType::I1) | HandleType::Literal(LlvmType::I1) => {
                    LlvmHandle::new_tmp_i1_register(name)
                }
            }),
        }
    }

    fn visit_print(&mut self, node: &mut ast::Print) -> VisitorResult {
        let inner_result = node.expression.accept(self);

        let preamble = inner_result.preamble
            + &match inner_result.result_handle {
                Some(handle) => match handle.handle_type {
                    HandleType::Register(LlvmType::F64) | HandleType::Literal(LlvmType::F64) => {
                        let element_ptr_variable = self.generate_tmp_variable();

                        format!(
                            "{} = getelementptr inbounds [3 x i8], [3 x i8]* @.fstr, i32 0, i32 0\ncall i32 (i8*, ...) @printf(i8* {}, double {})",
                            element_ptr_variable, element_ptr_variable, handle.llvm_name
                        )
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

    fn visit_while(&mut self, node: &mut ast::While) -> VisitorResult {
        let condition_result = node.condition.accept(self);
        let condition_result_handle = condition_result
            .result_handle
            .expect("Expected a result handle for condition of while statement");

        let body_result = node.body.accept(self);

        let (loop_label, body_label, loop_exit_label) = self.generate_loop_labels();

        let mut loop_setup_code = format!("br label %{}\n", loop_label)
            + &format!("{}:\n", loop_label)
            + &condition_result.preamble;

        match condition_result_handle.handle_type {
            HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                loop_setup_code = loop_setup_code
                    + &format!(
                        "br i1 {}, label %{}, label %{}\n",
                        condition_result_handle.llvm_name, body_label, loop_exit_label
                    );
            }
            _ => panic!("Expected a boolean handle for condition of if expression"),
        };

        let body_code = format!("{}:\n", body_label)
            + &body_result.preamble
            + &format!("br label %{}\n", loop_label);

        let loop_exit_code = format!("{}:\n", loop_exit_label);

        let preamble = loop_setup_code + &body_code + &loop_exit_code;

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    fn visit_block(&mut self, node: &mut ast::Block) -> VisitorResult {
        self.context.push_frame(true);
        let result = node.expression_list.accept(self);
        self.context.pop_frame();

        result
    }

    fn visit_un_op(&mut self, node: &mut ast::UnOp) -> VisitorResult {
        let inner_result = node.rhs.accept(self);
        if inner_result.result_handle.is_none() {
            panic!("Expected a result handle for operand of unary operator");
        }

        match inner_result.result_handle.as_ref().unwrap().handle_type {
            HandleType::Literal(LlvmType::F64) | HandleType::Register(LlvmType::F64) => {
                self.get_double_un_op_visitor_result(&node.op, inner_result)
            }
            HandleType::Literal(LlvmType::I1) | HandleType::Register(LlvmType::I1) => {
                self.get_boolean_un_op_visitor_result(&node.op, inner_result)
            }
        }
    }

    fn visit_variable(&mut self, node: &mut ast::Identifier) -> VisitorResult {
        let register_name = self.generate_tmp_variable();

        let variable = self
            .context
            .get_value(&node.id)
            .expect(format!("Variable {} not found, SA should have caught this", node.id).as_str());

        match variable.var_type {
            LlvmType::F64 => {
                let preamble = format!(
                    "{} = load double, double* {}, align 8\n",
                    register_name, variable.llvm_name
                );

                return VisitorResult {
                    preamble,
                    result_handle: Some(LlvmHandle::new_tmp_f64_register(register_name)),
                };
            }
            LlvmType::I1 => {
                let preamble = format!(
                    "{} = load i1, i1* {}, align 1\n",
                    register_name, variable.llvm_name
                );

                VisitorResult {
                    preamble,
                    result_handle: Some(LlvmHandle::new_tmp_i1_register(register_name)),
                }
            }
        }
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
