mod assignment;
mod bin_op;
mod block;
mod if_else;
mod print;
mod type_def;
mod un_op;
mod while_exp;

mod helpers {
    pub mod control_flow;
    pub mod variables;
}
use std::cell::Cell;

use std::collections::HashMap;
use std::string::String;

use crate::context::Context;
use crate::llvm_types::{HandleType, LlvmHandle, LlvmType};
use ast::typing::to_string;
use ast::{
    Definition, DefinitionVisitor, Expression, ExpressionVisitor, ListIndexing,
    VisitableDefinition, VisitableExpression,
};


pub struct VisitorResult {
    pub result_handle: Option<LlvmHandle>,
    pub preamble: String,
}

impl VisitorResult {
    /// Returns true if the result handle is None (i.e., the node does not produce a value).
    pub fn has_null_result(&self) -> bool {
        matches!(self.result_handle, None)
    }
}

/// Represents a variable in the code generation context.
/// Holds its LLVM type and the name of the LLVM register or pointer.
struct Variable {
    var_type: LlvmType,
    llvm_name: String,
}

impl Variable {
    /// Creates a new variable of type f64 (double).
    pub fn new_f64(llvm_name: String) -> Variable {
        Variable {
            var_type: LlvmType::F64,
            llvm_name,
        }
    }

    /// Creates a new variable of type i1 (boolean).
    pub fn new_i1(llvm_name: String) -> Variable {
        Variable {
            var_type: LlvmType::I1,
            llvm_name,
        }
    }

    /// Creates a new variable of type string (i8*).
    pub fn new_string(llvm_name: String) -> Variable {
        Variable {
            var_type: LlvmType::String,
            llvm_name,
        }
    }

    /// Creates a new variable of type object (pointer to struct or i8*).
    pub fn new_object(llvm_name: String) -> Variable {
        Variable {
            var_type: LlvmType::Object,
            llvm_name,
        }
    }

    pub fn new_list(llvm_name: String) -> Variable {
        Variable {
            var_type: LlvmType::List,
            llvm_name,
        }
    }
}

/// The main code generation visitor for the Hulk language.
///
/// This struct is responsible for traversing the AST and generating LLVM IR code.
/// It maintains context for variables, type members, function members, inheritance, and string constants.
pub struct GeneratorVisitor {
    /// Stores the names of the LLVM registers that store the pointers to the values of the variables defined in a given context.
    ///
    /// ## Warning
    /// To define variables, use the define_or_shadow method of this class
    context: Context<Variable>,

    /// Used to generate unique ids for temporary variables, irrespective of context.
    /// This way we don't need to worry about LLVM's requirement that %N names be sequential starting at 0 within the same context.
    tmp_variable_id: u32,
    tmp_counter: Cell<usize>,

    /// Allows shadowing variables or defining variables with the same name in different contexts.
    variable_ids: HashMap<String, u32>,

    /// Maps (type_name, member_name) to the member index in the struct.
    type_members_ids: HashMap<(String, String), u32>,
    /// Maps (type_name, member_name) to the member's type as a string.
    type_members_types: HashMap<(String, String), String>,
    /// Maps (type_name, function_name) to the LLVM function name in the vtable.
    function_member_names: HashMap<(String, String), String>,
    /// Maps type_name to its parent type (for inheritance).
    inherits: HashMap<String, String>,
    /// Maps type_name to the types of its constructor arguments.
    constructor_args_types: HashMap<String, Vec<String>>,
    /// Maps string literals to their LLVM global names.
    pub(crate) string_constants: Vec<String>,
    /// Counter for generating unique string constant names.
    _string_counter: u32,
    /// Maps (type_name, function_name) to the argument types for the function member.
    function_member_def_from_type_and_name: HashMap<(String, String, i32), Vec<String>>,
    /// Stores global string definitions for emission at the top of the LLVM IR.
    _global_string_definitions: Vec<String>,
    /// Maps (type_name, member_name) to the original type for the definition (for type resolution).
    original_type_for_definition: HashMap<(String, String), String>,

    /// Maps (type_name, function_name) to the LLVM function signature.
    function_member_signature_types: HashMap<(String, String), String>,
}

impl GeneratorVisitor {
    /// Creates a new `GeneratorVisitor` with empty context and all maps initialized.
    pub fn new() -> Self {
        GeneratorVisitor {
            context: Context::new_one_frame(),
            tmp_variable_id: 0,
            variable_ids: HashMap::new(),
            type_members_ids: HashMap::new(),
            type_members_types: HashMap::new(),
            function_member_names: HashMap::new(),
            inherits: HashMap::new(),
            constructor_args_types: HashMap::new(),
            string_constants: Vec::new(),
            _string_counter: 0,
            _global_string_definitions: Vec::new(),
            function_member_def_from_type_and_name: HashMap::new(),
            original_type_for_definition: HashMap::new(),
            tmp_counter: Cell::new(0),
            function_member_signature_types: HashMap::new(),
        }
    }

    /// Saves the mapping from member names to their indices for a given type.
    /// Used for struct field access in LLVM IR.
    fn _save_type_member_indices_from_defs(
        &mut self,
        type_name: &str,
        data_member_defs: &[ast::DataMemberDef],
    ) {
        for (i, data_member) in data_member_defs.iter().enumerate() {
            let member_id = data_member.identifier.id.clone();
            self.type_members_ids
                .insert((type_name.to_string(), member_id), i as u32);
        }
    }

    /// Saves the mapping from function member names to their LLVM names for a given type.
    /// Used for vtable method lookup.
    fn _save_function_member_names_from_defs(
        &mut self,
        type_name: &str,
        function_member_defs: &[ast::FunctionDef],
    ) {
        for func_def in function_member_defs.iter() {
            let func_id = func_def.identifier.id.clone();
            let llvm_func_name = format!("{}_{}", type_name, func_id);
            self.function_member_names
                .insert((type_name.to_string(), func_id), llvm_func_name);
        }
    }
}

// === ExpressionVisitor Implementation ===
// Implements code generation for all expression AST nodes.
// Each method emits LLVM IR for the corresponding AST node and returns a VisitorResult.
impl ExpressionVisitor<VisitorResult> for GeneratorVisitor {
    fn visit_block(&mut self, node: &mut ast::Block) -> VisitorResult {
        self.context.push_open_frame();
        let result =
            self.handle_block_items(&mut node.body_items, node.multiple_semicolon_terminated);
        self.context.pop_frame();

        result
    }

    fn visit_expression(&mut self, node: &mut ast::Expression) -> VisitorResult {
        node.accept(self)
    }

    fn visit_destructive_assignment(
        &mut self,
        node: &mut ast::DestructiveAssignment,
    ) -> VisitorResult {
        let expression_result = node.rhs.accept(self);
        let mut preamble = expression_result.preamble;

        let exp_result_handle = expression_result.result_handle.expect(
            "Variable must be assigned to non-null expression result, SA should've caught this",
        );

        match node.lhs.as_mut() {
            Expression::Variable(var) => {
                // Handle regular variable assignment
                let var_llvm_name = &self
                    .context
                    .get_value(&var.id)
                    .expect(&format!(
                        "Variable {} not found, SA should have caught this",
                        var.id
                    ))
                    .llvm_name;

                preamble += &self.store_statement(
                    &exp_result_handle.llvm_name,
                    &var_llvm_name,
                    &exp_result_handle.handle_type.inner_type(),
                );
            }
            Expression::DataMemberAccess(data_member_access) => {
                // Handle data member assignment
                let object_result = data_member_access.object.accept(self);
                preamble += &object_result.preamble;
                
                let object_ptr = object_result
                    .result_handle
                    .expect("Object must have a result")
                    .llvm_name;
                let object_type = data_member_access.obj_type.clone();

                let type_name = match &object_type {
                    Some(ty) => to_string(&Some(ty.clone())),
                    None => panic!("Object type not found for data member access"),
                };
                let member_id = data_member_access.member.id.clone();

                if let Some(idx) = self
                    .type_members_ids
                    .get(&(type_name.clone(), member_id.clone()))
                {
                    let field_index = idx.clone();
                    
                    // Get pointer to the member field
                    let member_ptr = self.generate_tmp_variable();
                    let gep_instr = format!(
                        "  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                        member_ptr, type_name, type_name, object_ptr, field_index
                    );
                    preamble += &gep_instr;

                    // Store the new value to the member
                    let member_type = data_member_access.member.info.ty.clone();
                    let llvm_type = match &member_type {
                        Some(ty) => match ty {
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => LlvmType::F64,
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => LlvmType::I1,
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => LlvmType::String,
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => LlvmType::Object,
                            _ => LlvmType::Object,
                        },
                        None => LlvmType::Object,
                    };

                    preamble += &self.store_statement(
                        &exp_result_handle.llvm_name,
                        &member_ptr,
                        &llvm_type,
                    );
                } else {
                    panic!(
                        "Data member '{}' not found in type '{}'",
                        member_id, type_name
                    );
                }
            }
            _ => {
                panic!("Unsupported left-hand side expression type for destructive assignment");
            }
        }

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

    fn visit_let_in(&mut self, node: &mut ast::LetIn) -> VisitorResult {
        self.context.push_open_frame();

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

    fn visit_while(&mut self, node: &mut ast::While) -> VisitorResult {
        let condition_result = node.condition.accept(self);
        let body_result = node.body.accept(self);

        self.handle_while(condition_result, body_result)
    }

    fn visit_for(&mut self, _node: &mut ast::For) -> VisitorResult {
        todo!()
    }

    fn visit_un_op(&mut self, node: &mut ast::UnOp) -> VisitorResult {
        let inner_result = node.rhs.accept(self);

        self.handle_un_op(inner_result, &node.op)
    }

    fn visit_data_member_access(&mut self, node: &mut ast::DataMemberAccess) -> VisitorResult {
        let object_result = node.object.accept(self);
        let mut preamble = object_result.preamble;
        let object_ptr = object_result
            .result_handle
            .expect("Object must have a result")
            .llvm_name;
        let object_type = node.obj_type.clone();

        // println!("Object type: {:?}", object_type);
        // println!("Object pointer: {}", object_ptr);
        // println!("Member access: {}", node.member.id);

        let member_type = node.member.info.ty.clone();

        let llvm_type = match &member_type {
            Some(ty) => match ty {
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => LlvmType::F64,
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => LlvmType::I1,
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => LlvmType::String,
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => LlvmType::Object,
                _ => LlvmType::Object,
            },
            None => LlvmType::Object,
        };
        // println!("{}", member_type.unwrap().to_string());
        // println!("{}", llvm_type.llvm_type_str());
        // println!("{}", node.member.id);
        let type_name = match &object_type {
            Some(ty) => to_string(&Some(ty.clone())),

            None => panic!("Object type not found for data member access"),
        };
        let member_id = node.member.id.clone();

        let result_var = self.generate_tmp_variable();

        // println!("Looking for member '{}' in type '{}'", member_id, type_name);

        if let Some(idx) = self
            .type_members_ids
            .get(&(type_name.clone(), member_id.clone()))
        {
            let field_index = idx.clone();
            // println!("Found member in index'{}'", field_index);
            let gep_instr = format!(
                "  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                result_var, type_name, type_name, object_ptr, field_index
            );
            preamble += &gep_instr;

            if matches!(llvm_type, LlvmType::F64 | LlvmType::I1) {
                let load_var = self.generate_tmp_variable();
                let load_instr = format!(
                    "  {} = load {}, {}* {}, align 8\n",
                    load_var,
                    llvm_type.llvm_type_str(),
                    llvm_type.llvm_type_str(),
                    result_var
                );
                preamble += &load_instr;
                return VisitorResult {
                    preamble,
                    result_handle: Some(LlvmHandle {
                        handle_type: HandleType::Register(llvm_type),
                        llvm_name: load_var,
                    }),
                };
            } else if matches!(llvm_type, LlvmType::Object) {
                let load_var = self.generate_tmp_variable();
                let load_instr =
                    format!("  {} = load i8*, i8** {}, align 8\n", load_var, result_var);
                preamble += &load_instr;
                return VisitorResult {
                    preamble,
                    result_handle: Some(LlvmHandle {
                        handle_type: HandleType::Register(LlvmType::Object),
                        llvm_name: load_var,
                    }),
                };
            }
        } else {
            panic!(
                "Data member '{}' not found in type '{}'",
                member_id, type_name
            );
        }

        VisitorResult {
            preamble,
            result_handle: Some(LlvmHandle {
                handle_type: HandleType::Register(llvm_type),
                llvm_name: result_var,
            }),
        }
    }

    fn visit_function_member_access(
        &mut self,
        node: &mut ast::FunctionMemberAccess,
    ) -> VisitorResult {
        let mut preamble = String::new();

        let object_result = node.object.accept(self);
        preamble += &object_result.preamble;
        let object_handle = object_result
            .result_handle
            .expect("Object for method call must have a result");
        let object_ptr_name = object_handle.llvm_name.clone();

        let object_ast_type_name = match &node.obj_type {
            Some(ty) => match ty {
                ast::typing::Type::Defined(defined_type) => defined_type.id.clone(),
                _ => panic!("Object type for method call must be a defined type name"),
            },
            None => panic!("Object type not found for function member access"),
        };

        let func_name_in_ast = node.member.identifier.id.clone();
        let mut current_type = object_ast_type_name.clone();
        let mut current_object_ptr = object_ptr_name.clone();

        // Prepare call signature and arguments
        let call_ret_type_str = match &node.member.identifier.info.ty {
            Some(ty) => self.llvm_type_str_from_ast_type(ty),
            None => "void".to_string(),
        };

        let mut call_param_llvm_types_for_sig = Vec::new();
        let mut call_args_values_with_llvm_types = Vec::new();
        // Always push the self pointer as i8*
        // call_param_llvm_types_for_sig.push("i8*".to_string());
        // call_args_values_with_llvm_types.push(format!("i8* {}", object_ptr_name));
        // Instead, use the correct type for self pointer
        call_param_llvm_types_for_sig.push(format!("%{}_type*", current_type));
        call_args_values_with_llvm_types.push(format!("%{}_type* {}", current_type, object_ptr_name));

        // Look up argument types from the context map
        let mut arg_types: Vec<String> = Vec::new();
        for ((type_name, method_name, _index), arg_types_value) in
            &self.function_member_def_from_type_and_name
        {
            if type_name == &current_type && method_name == &func_name_in_ast {
                arg_types = arg_types_value.clone();
            }
        }

        // Iterate and call methods that require `&mut self`
        for (arg_expr, arg_llvm_type_str) in node.member.arguments.iter_mut().zip(arg_types.iter())
        {
            let arg_result = arg_expr.accept(self);
            preamble += &arg_result.preamble;
            let arg_handle = arg_result
                .result_handle
                .expect("Function member argument must have a result");
            call_param_llvm_types_for_sig.push(arg_llvm_type_str.clone());
            call_args_values_with_llvm_types
                .push(format!("{} {}", arg_llvm_type_str, arg_handle.llvm_name));
        }

        let func_signature_ptr_type_for_load = format!(
            "{} ({})*",
            call_ret_type_str,
            call_param_llvm_types_for_sig.join(", ")
        );

        // Inheritance-aware vtable lookup
        loop {
            let vtable_type_name = format!("%{}_vtable_type", current_type);
            let object_llvm_type_name = format!("%{}_type", current_type);

            // Cast object pointer to current type if needed
            let object_typed_ptr = self.generate_tmp_variable();
            preamble += &format!(
                "  {} = bitcast i8* {} to {}*\n",
                object_typed_ptr, current_object_ptr, object_llvm_type_name
            );

            // Get vtable pointer
            let vtable_ptr_ptr = self.generate_tmp_variable();
            preamble += &format!(
                "  {} = getelementptr inbounds {}, {}* {}, i32 0, i32 0\n",
                vtable_ptr_ptr, object_llvm_type_name, object_llvm_type_name, object_typed_ptr
            );
            let vtable_ptr = self.generate_tmp_variable();
            preamble += &format!(
                "  {} = load {}*, {}** {}, align 8\n",
                vtable_ptr, vtable_type_name, vtable_type_name, vtable_ptr_ptr
            );
            let func_ptr_location_in_vtable = self.generate_tmp_variable();
            // Get the correct function pointer type for the vtable entry
            let vtable_func_ptr_type = func_signature_ptr_type_for_load.clone();
            preamble += &format!(
                "  {} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}\n",
                func_ptr_location_in_vtable,
                vtable_type_name,
                vtable_type_name,
                vtable_ptr,
                self.function_member_names.get(&(current_type.clone(), func_name_in_ast.clone())).unwrap()
            );
            let loaded_func_ptr = self.generate_tmp_variable();
            preamble += &format!(
                "  {} = load {}, {}* {}, align 8\n",
                loaded_func_ptr,
                vtable_func_ptr_type,
                vtable_func_ptr_type,
                func_ptr_location_in_vtable
            );

            let result_reg: String;
            if call_ret_type_str != "void" {
                result_reg = self.generate_tmp_variable();
                preamble += &format!(
                    "  {} = call {} {}({})\n",
                    result_reg,
                    call_ret_type_str,
                    loaded_func_ptr,
                    call_args_values_with_llvm_types.join(", ")
                );
            } else {
                result_reg = "".to_string();
                preamble += &format!(
                    "  call void {}({})\n",
                    loaded_func_ptr,
                    call_args_values_with_llvm_types.join(", ")
                );
            }

            let result_handle = if call_ret_type_str != "void" {
                let ast_ret_type = node
                    .member
                    .identifier
                    .info
                    .ty
                    .as_ref()
                    .expect("Return type must be known for non-void");
                Some(LlvmHandle {
                    handle_type: HandleType::Register(
                        self.llvm_type_from_ast_type(ast_ret_type),
                    ),
                    llvm_name: result_reg,
                })
            } else {
                None
            };

            return VisitorResult {
                preamble,
                result_handle,
            };
        }
    }

    fn visit_function_call(&mut self, node: &mut ast::FunctionCall) -> VisitorResult {
        if node.identifier.id == "print" && node.arguments.len() == 1 {
            let inner_result = node.arguments[0].accept(self);
            return self.handle_print(inner_result);
        }

        let mut preamble = String::new();
        let mut arg_values = Vec::new();
        let mut arg_types = Vec::new();
        for arg in node.arguments.iter_mut() {
            let arg_result = arg.accept(self);
            preamble += &arg_result.preamble;
            let handle = arg_result
                .result_handle
                .expect("Function argument must have a result");
            arg_values.push(handle.llvm_name);

            let arg_type = match arg {
                ast::Expression::NumberLiteral(_) => "double".to_string(),
                ast::Expression::BooleanLiteral(_) => "i1".to_string(),
                ast::Expression::StringLiteral(_) => "i8*".to_string(),
                ast::Expression::Variable(id) => match &id.info.ty {
                    Some(ty) => match ty {
                        ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => {
                            "double".to_string()
                        }
                        ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => {
                            "i1".to_string()
                        }
                        ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => {
                            "i8*".to_string()
                        }
                        ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => {
                            "i8*".to_string()
                        }
                        ast::typing::Type::Defined(name) => format!("%{}_type*", name.id.clone()),
                        _ => "i8*".to_string(),
                    },
                    None => "i8*".to_string(),
                },
                _ => "i8*".to_string(),
            };
            arg_types.push(arg_type);
        }

        let ret_type = match &node.identifier.info.ty {
            Some(ty) => match ty {
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => {
                    "double".to_string()
                }
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => "i1".to_string(),
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => "i8*".to_string(),
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => "i8*".to_string(),
                ast::typing::Type::Defined(name) => format!("%{}_type*", name.id.clone()),
                _ => "i8*".to_string(),
            },
            None => "i8*".to_string(),
        };

        let result_var = self.generate_tmp_variable();
        let call_args = arg_values
            .iter()
            .zip(arg_types.iter())
            .map(|(a, t)| format!("{} {}", t, a))
            .collect::<Vec<_>>()
            .join(", ");
        preamble += &format!(
            "  {} = call {} @{}({})\n",
            result_var, ret_type, node.identifier.id, call_args
        );

        let handle_type = match ret_type.as_str() {
            "double" => HandleType::Register(LlvmType::F64),
            "i1" => HandleType::Register(LlvmType::I1),
            "i8*" => HandleType::Register(LlvmType::String),
            s if s.ends_with("*") && s.starts_with("%") => HandleType::Register(LlvmType::Object),
            _ => HandleType::Register(LlvmType::Object),
        };

        VisitorResult {
            preamble,
            result_handle: Some(LlvmHandle {
                handle_type,
                llvm_name: result_var,
            }),
        }
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

    fn visit_string_literal(&mut self, node: &mut ast::StringLiteral) -> VisitorResult {
        let value = node.string.clone();
        let a = self.generate_tmp_variable();
        let tmp_var = &a[1..]; // Remove the % prefix for global name
        let global_str_name = format!("{}_str", tmp_var);
        let str_len = value.len();

        // Define the global string constant (with null terminator)
        let global_str_code = format!(
            "@{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n",
            global_str_name,
            str_len + 1,
            value
        );

        self.string_constants.push(global_str_code.clone());

        // Create a heap-allocated string using malloc
        let local_str_var = self.generate_tmp_variable();
        let malloc_code = format!(
            "{} = call i8* @malloc(i64 {})\n",
            local_str_var,
            str_len + 1
        );

        // Get pointer to the global string constant
        let global_ptr_var = self.generate_tmp_variable();
        let global_gep_code = format!(
            "{} = getelementptr inbounds [{} x i8], [{} x i8]* @{}, i64 0, i64 0\n",
            global_ptr_var,
            str_len + 1,
            str_len + 1,
            global_str_name
        );

        // Copy the string to heap variable using strcpy
        let strcpy_code = format!(
            "call i8* @strcpy(i8* {}, i8* {})\n",
            local_str_var, global_ptr_var
        );

        VisitorResult {
            preamble: malloc_code + &global_gep_code + &strcpy_code,
            result_handle: Some(LlvmHandle::new_string_register(local_str_var)), // Return pointer to heap string
        }
    }
    fn visit_list_literal(&mut self, node: &mut ast::ListLiteral) -> VisitorResult {
        let type_name = match &node.list_type {
            Some(ty) => to_string(&Some(ty.clone())),
            None => panic!("Object type not found for data member access"),
        };

        let mut preamble = String::new();
        let mut element_handles = Vec::new();

        for element in node.elements.iter_mut() {
            let e_result = element.accept(self);
            preamble += &e_result.preamble;
            if let Some(handle) = e_result.result_handle {
                element_handles.push(handle);
            }
        }

        let list_len = element_handles.len();
        let tmp_var_id = self.tmp_counter.get();
        self.tmp_counter.set(tmp_var_id + 1);

        let llvm_elem_type = match &node.list_type {
            Some(ast_type) => self.llvm_type_str_from_ast_type(ast_type),
            None => "i64".to_string(), // fallback, though shouldn't happen
        };
        print!("heyyyyy {llvm_elem_type}");
        // Allocate memory for the array using malloc
        // e.g., %list_ptr = call i8* @malloc(i64 size)
        let type_size = self.llvm_type_size(&llvm_elem_type);
        let total_size = type_size * list_len;
        let ptr_var = format!("%list_ptr_{}", tmp_var_id);
        preamble += &format!(
            "{ptr_var} = call i8* @malloc(i64 {})\n",
            total_size
        );
        // Bitcast to the appropriate pointer type
        let array_ptr = format!("%casted_list_ptr_{}", tmp_var_id);
        preamble += &format!(
            "{array_ptr} = bitcast i8* {ptr_var} to {elem_type}*\n",
            ptr_var=ptr_var,
            array_ptr=array_ptr,
            elem_type=llvm_elem_type
        );

        // Store elements into the array memory
        for (i, handle) in element_handles.iter().enumerate() {
            let elem_ptr = format!("%elem_ptr_{}_{}", tmp_var_id, i);
            preamble += &format!(
                "{elem_ptr} = getelementptr inbounds {elem_type}, {elem_type}* {array_ptr}, i64 {idx}\n",
                elem_ptr=elem_ptr,
                elem_type=llvm_elem_type,
                array_ptr=array_ptr,
                idx=i
            );
            preamble += &format!(
                "store {elem_type} {value}, {elem_type}* {elem_ptr}\n",
                elem_type=llvm_elem_type,
                value=handle.llvm_name
            );
        }

        // Return the pointer to the start of the list as the handle
        VisitorResult {
            preamble,
            result_handle: Some(crate::llvm_types::LlvmHandle::new_list_register(
                array_ptr,
            )),
        }
    }


    fn visit_empty_expression(&mut self) -> VisitorResult {
        VisitorResult {
            preamble: "".to_string(),
            result_handle: None,
        }
    }

    fn visit_return_statement(&mut self, node: &mut ast::ReturnStatement) -> VisitorResult {
        let mut preamble = String::new();

        let expr_result = node.expression.accept(self);
        preamble += &expr_result.preamble;
        let result_handle = expr_result.result_handle;

        VisitorResult {
            preamble,
            result_handle,
        }
    }

    fn visit_new_expr(&mut self, node: &mut ast::NewExpr) -> VisitorResult {
        let mut preamble = String::new();
        let mut arg_handles = Vec::new();
        let mut arg_types = Vec::new();
        // Use constructor_args_types to get the expected types for the constructor
        let expected_types = self
            .constructor_args_types
            .get(&node.type_name)
            .cloned();
        for (i, arg) in node.arguments.iter_mut().enumerate() {
            let arg_result = arg.accept(self);
            preamble += &arg_result.preamble;
            let handle = arg_result
                .result_handle
                .expect("Constructor argument must have a result");
            arg_handles.push(handle.llvm_name);
            // Use the expected type from constructor_args_types if available
            let arg_type = if let Some(ref types) = expected_types {
                if i < types.len() {
                    types[i].clone()
                } else {
                    "i8*".to_string() // fallback if out of bounds
                }
            } else {
                "i8*".to_string() // fallback if not found
            };
            arg_types.push(arg_type);
        }

        let result_var = self.generate_tmp_variable();
        let call_args = arg_handles
            .iter()
            .zip(arg_types.iter())
            .map(|(a, t)| format!("{} {}", t, a))
            .collect::<Vec<_>>()
            .join(", ");
        preamble += &format!(
            "  {} = call %{}_type* @{}_new({})\n",
            result_var, node.type_name, node.type_name, call_args
        );
        VisitorResult {
            preamble,
            result_handle: Some(crate::llvm_types::LlvmHandle::new_object_register(
                result_var,
            )),
        }
    }

    fn visist_list_indexing(&mut self, node: &mut ListIndexing) -> VisitorResult {
        let mut preamble = String::new();

        // Visit the list expression to get the list handle
        let list_result = node.list.accept(self);
        preamble += &list_result.preamble;
        let list_handle = match list_result.result_handle {
            Some(h) => h,
            None => panic!("Expected handle for list expression"),
        };

        // Visit the index expression to get the index value
        let index_result = node.index.accept(self);
        preamble += &index_result.preamble;
        let index_handle = match index_result.result_handle {
            Some(h) => h,
            None => panic!("Expected handle for index expression"),
        };

        let elem_type = match &node.list_type {
            Some(ast_type) => self.llvm_type_str_from_ast_type(ast_type),
            None => panic!("Expected list type for list indexing"),
        };

        let tmp_var_id = self.tmp_counter.get();
        self.tmp_counter.set(tmp_var_id + 1);

        let casted_list_ptr = if list_handle.llvm_name.contains("list_ptr") {
            list_handle.llvm_name.clone()
        } else {
            let casted_ptr = format!("%list_elem_ptr_{}", tmp_var_id);
            preamble += &format!(
                "  {} = bitcast i8* {} to {}*\n",
                casted_ptr, list_handle.llvm_name, elem_type
            );
            casted_ptr
        };

        // Compute pointer to the indexed element
        let elem_ptr = format!("%elem_ptr_{}", tmp_var_id);
        preamble += &format!(
            "  {} = getelementptr inbounds {}, {}* {}, i64 {}\n",
            elem_ptr, elem_type, elem_type, casted_list_ptr, index_handle.llvm_name.split('.').next().unwrap()
        );

        // Load the value from the array
        let loaded_val = format!("%loaded_elem_{}", tmp_var_id);
        preamble += &format!(
            "  {} = load {}, {}* {}\n",
            loaded_val, elem_type, elem_type, elem_ptr
        );

        // Create the appropriate handle type based on the element type
        let handle_type = match elem_type.as_str() {
            "double" => HandleType::Register(LlvmType::F64),
            "i1" => HandleType::Register(LlvmType::I1),
            "i8*" => HandleType::Register(LlvmType::String),
            s if s.ends_with("*") && s.starts_with("%") => HandleType::Register(LlvmType::Object),
            _ => HandleType::Register(LlvmType::F64),
        };

        VisitorResult {
            preamble,
            result_handle: Some(LlvmHandle {
                handle_type,
                llvm_name: loaded_val,
            }),
        }
    }
}

// === DefinitionVisitor Implementation ===
// Implements code generation for all definition AST nodes (types, functions, constants, protocols).
impl DefinitionVisitor<VisitorResult> for GeneratorVisitor {
    fn visit_definition(&mut self, node: &mut Definition) -> VisitorResult {
        node.accept(self)
    }

    fn visit_type_def(&mut self, node: &mut ast::TypeDef) -> VisitorResult {
        use crate::visitor::type_def::{
            generate_constructor, generate_method_definitions, generate_object_struct_type,
            generate_vtable_type,
        };
        let mut preamble = String::new();
        // VTable
        preamble += &generate_vtable_type(self, node);
        // Struct
        let struct_str = generate_object_struct_type(self, node);
        preamble += &struct_str;
        // Collect field types for constructor
        let _type_name = &node.name.id;

        // Constructor
        preamble += &generate_constructor(self, node);
        // Methods
        preamble += &generate_method_definitions(self, node);

        // println!("Type members IDs map contents:");
        // for ((type_name, member_name), index) in &self.type_members_ids {
        //     println!(
        //         "Type: {}, Member: {}, Index: {}",
        //         type_name, member_name, index
        //     );
        // }

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    fn visit_function_def(&mut self, node: &mut ast::GlobalFunctionDef) -> VisitorResult {
        let mut preamble = String::new();
        let func_name = &node.function_def.identifier.id;

        let return_type = match &node.function_def.identifier.info.ty {
            Some(ty) => match ty {
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => {
                    "double".to_string()
                }
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => "i1".to_string(),
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => "i8*".to_string(),
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => "i8*".to_string(),
                ast::typing::Type::Defined(name) => format!("%{}_type*", name.id.clone()),
                _ => "void".to_string(),
            },
            None => "void".to_string(),
        };

        preamble += &format!("define {} @{}(", return_type, func_name);

        for (i, param) in node.function_def.parameters.iter().enumerate() {
            if i > 0 {
                preamble += ", ";
            }
            let llvm_type = match param.info.ty.clone() {
                Some(ty) => match ty {
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => {
                        "double".to_string()
                    }
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => "i1".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => {
                        "i8*".to_string()
                    }
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => {
                        "i8*".to_string()
                    }
                    ast::typing::Type::Defined(name) => format!("%{}_type*", name.id.clone()),
                    _ => "i8*".to_string(),
                },
                None => "i8*".to_string(),
            };
            preamble += &format!("{} %{}", llvm_type, param.id);
        }
        preamble += ") {\n";
        preamble += "entry:\n";

        let old_context = std::mem::replace(&mut self.context, Context::new_one_frame());

        for param in &node.function_def.parameters {
            let llvm_type = match param.info.ty.clone() {
                Some(ty) => match ty {
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => LlvmType::F64,
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => LlvmType::I1,
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => {
                        LlvmType::String
                    }
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => {
                        LlvmType::Object
                    }
                    ast::typing::Type::Defined(_) => LlvmType::Object,
                    _ => LlvmType::Object,
                },
                None => LlvmType::Object,
            };
            let param_ptr = self.generate_tmp_variable();
            preamble += &self.alloca_statement(&param_ptr, &llvm_type);
            preamble += &self.store_statement(&format!("%{}", param.id), &param_ptr, &llvm_type);
            match llvm_type {
                LlvmType::F64 => self
                    .context
                    .define(param.id.clone(), Variable::new_f64(param_ptr)),
                LlvmType::I1 => self
                    .context
                    .define(param.id.clone(), Variable::new_i1(param_ptr)),
                LlvmType::String => self
                    .context
                    .define(param.id.clone(), Variable::new_string(param_ptr)),
                LlvmType::Object => self
                    .context
                    .define(param.id.clone(), Variable::new_object(param_ptr)),
                _ => panic!("Unsuported type")
            };
        }

        let body_result = match &mut node.function_def.body {
            ast::FunctionBody::ArrowExpression(arrow_exp) => {
                let exp_result = arrow_exp.expression.accept(self);
                exp_result
            }
            ast::FunctionBody::Block(block) => {
                let block_result = self.visit_block(block);
                block_result
            }
        };

        preamble += &body_result.preamble;

        if return_type != "void" {
            if let Some(result_handle) = &body_result.result_handle {
                // if return_type == "i8*" && result_handle.llvm_name.starts_with("%") {
                //     let load_var = self.generate_tmp_variable();
                //     preamble += &format!(
                //         "  {} = load i8*, i8** {}, align 8\n  ret i8* {}\n",
                //         load_var, result_handle.llvm_name, load_var
                //     );
                // } else {
                preamble += &format!("  ret {} {}\n", return_type, result_handle.llvm_name);
                // }
            } else {
                match return_type.as_str() {
                    "double" => preamble += "  ret double 0.0\n",
                    "i1" => preamble += "  ret i1 0\n",
                    "i8*" => preamble += "  ret i8* null\n",
                    _ if return_type.ends_with("*") => {
                        preamble += &format!("  ret {} null\n", return_type);
                    }
                    _ => preamble += "  ret void\n",
                }
            }
        } else {
            preamble += "  ret void\n";
        }

        preamble += "}\n\n";

        let _function_context = std::mem::replace(&mut self.context, old_context);

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    fn visit_constant_def(&mut self, node: &mut ast::ConstantDef) -> VisitorResult {
        let mut preamble = String::new();
        let constant_name = &node.identifier.id;

        let init_result = node.initializer_expression.accept(self);
        preamble += &init_result.preamble;

        let init_value = init_result
            .result_handle
            .expect("Constant initializer must have a result");

        match init_value.handle_type.inner_type() {
            LlvmType::F64 => {
                preamble += &format!(
                    "@{} = constant double {}\n\n",
                    constant_name, init_value.llvm_name
                );
            }
            LlvmType::I1 => {
                preamble += &format!(
                    "@{} = constant i1 {}\n\n",
                    constant_name, init_value.llvm_name
                );
            }
            LlvmType::String => {
                if init_value.llvm_name.starts_with("\"") {
                    let string_content = &init_value.llvm_name[1..init_value.llvm_name.len() - 1];
                    let byte_length = string_content.len() + 1;

                    preamble += &format!(
                        "@{}.str = private constant [{} x i8] c\"{}\\00\", align 1\n",
                        constant_name, byte_length, string_content
                    );
                    preamble += &format!(
                        "@{} = constant i8* getelementptr inbounds ([{} x i8], [{} x i8]* @{}.str, i64 0, i64 0)\n\n",
                        constant_name, byte_length, byte_length, constant_name
                    );
                } else {
                    preamble += &format!(
                        "@{} = constant i8* {}\n\n",
                        constant_name, init_value.llvm_name
                    );
                }
            }
            LlvmType::Object => {
                preamble += &format!(
                    "@{} = constant i8* {}\n\n",
                    constant_name, init_value.llvm_name
                );
            }
            _ => panic!("Unsuported type")
        }

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }

    fn visit_protocol_def(&mut self, node: &mut ast::ProtocolDef) -> VisitorResult {
        let mut preamble = String::new();
        let protocol_name = &node.name.id;

        preamble += &format!("%{}_vtable_type = type {{\n", protocol_name);

        for (i, func_sig) in node.function_signatures.iter().enumerate() {
            let _method_name = &func_sig.identifier.id;

            preamble += "  i8* (i8*";

            for _ in &func_sig.parameters {
                preamble += ", double";
            }
            preamble += ")*";

            if i < node.function_signatures.len() - 1 {
                preamble += ",\n";
            } else {
                preamble += "\n";
            }
        }

        preamble += "}\n\n";

        preamble += &format!("%{}_interface = type {{\n", protocol_name);
        preamble += "  i8*,                    ; Object pointer\n";
        preamble += &format!("  %{}_vtable_type*  ; VTable pointer\n", protocol_name);
        preamble += "}\n\n";

        VisitorResult {
            preamble,
            result_handle: None,
        }
    }
}

// === GeneratorVisitor Helper Methods ===
// Provides helpers for type conversion, LLVM type string generation, and type size/alignment.
impl GeneratorVisitor {
    fn llvm_type_from_ast_type(&self, ast_type: &ast::typing::Type) -> LlvmType {
        match ast_type {
            ast::typing::Type::BuiltIn(bt) => match bt {
                ast::typing::BuiltInType::Number => LlvmType::F64,
                ast::typing::BuiltInType::Bool => LlvmType::I1,
                ast::typing::BuiltInType::String => LlvmType::String,
                ast::typing::BuiltInType::Object => LlvmType::Object,
            },
            ast::typing::Type::Defined(_type_name) => LlvmType::Object,
            ast::typing::Type::Iterable(_inner_type_box) => LlvmType::Object,
            ast::typing::Type::Functor(_functor_type) => LlvmType::Object,
        }
    }

    fn llvm_type_str_from_ast_type(&self, ast_type: &ast::typing::Type) -> String {
        match ast_type {
            ast::typing::Type::BuiltIn(bt) => match bt {
                ast::typing::BuiltInType::Number => "double".to_string(),
                ast::typing::BuiltInType::Bool => "i1".to_string(),
                ast::typing::BuiltInType::String => "i8*".to_string(),
                ast::typing::BuiltInType::Object => "i8*".to_string(),
            },
            ast::typing::Type::Defined(type_name) => {
                format!("%{}_type*", type_name.id)
            }
            ast::typing::Type::Iterable(inner_type_box) => {
                format!(
                    "{}",
                    self.llvm_type_str_from_ast_type(inner_type_box.as_ref())
                )
            }
            _ => panic!("NO implemented type"),
        }
    }

    // Helper to get LLVM type size (for struct size calculation)
    fn llvm_type_size(&self, llvm_type: &str) -> usize {
        match llvm_type {
            "double" => 8,
            "i32" => 4,
            "i1" => 1,
            "i8*" => 8,
            _ if llvm_type.ends_with("*") => 8,
            _ => 8,
        }
    }
    // Helper to get LLVM type alignment
    fn _llvm_type_align(&self, llvm_type: &str) -> usize {
        match llvm_type {
            "double" => 8,
            "i32" => 4,
            "i1" => 1,
            "i8*" => 8,
            _ if llvm_type.ends_with("*") => 8,
            _ => 8,
        }
    }
}
