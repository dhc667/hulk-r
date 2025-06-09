mod assignment;
mod bin_op;
mod block;
mod if_else;
mod print;
mod un_op;
mod while_exp;

mod helpers {
    pub mod control_flow;
    pub mod variables;
}

use std::any::Any;
use std::collections::HashMap;

use crate::context::Context;
use crate::llvm_types::{LlvmHandle, LlvmType, HandleType};
use ast::{Expression, ExpressionVisitor, VisitableExpression, Definition, DefinitionVisitor, VisitableDefinition, BlockBodyItem, ListIndexing};
use ast::typing::to_string;

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
    
    pub fn new_string(llvm_name: String) -> Variable {
        Variable {
            var_type: LlvmType::String,
            llvm_name,
        }
    }
    
    pub fn new_object(llvm_name: String) -> Variable {
        Variable {
            var_type: LlvmType::Object,
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
    variable_ids: HashMap<String, u32>,

    type_members_ids: HashMap<(String,String), u32>,
    function_member_names: HashMap<(String, String), String>,
    inherits: HashMap<String, String>,
    constructor_args_types: HashMap<String, Vec<String>>,
}

impl GeneratorVisitor {
    pub fn new() -> Self {
        GeneratorVisitor {
            context: Context::new_one_frame(),
            tmp_variable_id: 0,
            variable_ids: HashMap::new(),
            type_members_ids: HashMap::new(),
            function_member_names: HashMap::new(),
            inherits: HashMap::new(),
            constructor_args_types: HashMap::new(),
        }
    }

    fn save_type_member_indices_from_defs(&mut self, type_name: &str, data_member_defs: &[ast::DataMemberDef]) {
        for (i, data_member) in data_member_defs.iter().enumerate() {
            let member_id = data_member.identifier.id.clone();
            self.type_members_ids.insert((type_name.to_string(), member_id), i as u32);
        }
    }

    fn save_function_member_names_from_defs(&mut self, type_name: &str, function_member_defs: &[ast::FunctionDef]) {
        for func_def in function_member_defs.iter() {
            let func_id = func_def.identifier.id.clone();
            let llvm_func_name = format!("{}_{}", type_name, func_id);
            self.function_member_names.insert((type_name.to_string(), func_id), llvm_func_name);
        }
    }
}

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
            "Variable must be dassigned to non-null expression result, SA should've caught this",
        );

        let variable = match node.lhs.as_ref() {
            Expression::Variable(var) => var,
            _ => todo!()
        };

        let var_llvm_name = &self
            .context
            .get_value(&variable.id)
            .expect(
                format!(
                    "Variable {} not found, SA should have caught this",
                    variable.id
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

    fn visit_for(&mut self, node: &mut ast::For) -> VisitorResult {
        todo!()
    }

    fn visit_un_op(&mut self, node: &mut ast::UnOp) -> VisitorResult {
        let inner_result = node.rhs.accept(self);

        self.handle_un_op(inner_result, &node.op)
    }

    fn visit_data_member_access(&mut self, node: &mut ast::DataMemberAccess) -> VisitorResult {
        let object_result = node.object.accept(self);
        let mut preamble = object_result.preamble;
        let object_ptr = object_result.result_handle.expect("Object must have a result").llvm_name;
        let object_type = node.obj_type.clone();

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
        println!("{}",member_type.unwrap().to_string());
        println!("{}",llvm_type.llvm_type_str());
        println!("{}",node.member.id);
        let type_name = match &object_type {
            Some(ty) => to_string(&Some(ty.clone())),
            
            None => panic!("Object type not found for data member access"),
        };
        let member_id = node.member.id.clone();


        let result_var = self.generate_tmp_variable();

        if let Some(idx) = self.type_members_ids.get(&(type_name.clone(), member_id.clone())) {
            let field_index = idx.clone();
            
            let gep_instr = match llvm_type {
                LlvmType::F64 | LlvmType::I1 => {
                    format!(
                        "  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                        result_var,
                        type_name,
                        type_name,
                        object_ptr,
                        field_index
                    )
                }
                LlvmType::Object => {
                    format!(
                        "  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                        result_var,
                        type_name,
                        type_name,
                        object_ptr,
                        field_index
                    )
                }
                _ => panic!("Unsupported type for data member access: {}", llvm_type.llvm_type_str()),
            };
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
                let load_instr = format!(
                    "  {} = load i8*, i8** {}, align 8\n",
                    load_var,
                    result_var
                );
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
            let mut current_type = type_name.clone();
            let mut current_object_ptr = object_ptr.clone();
            loop {
                let super_field_ptr = self.generate_tmp_variable();
                let super_field_load = self.generate_tmp_variable();
                if let Some(search_father) = self.inherits.get(&current_type) {
                    if let Some(idx) = self.type_members_ids.get(&(search_father.clone(), member_id.clone())) {
                        let field_index = idx.clone();

                        let gep_instr = match llvm_type {
                            LlvmType::F64 | LlvmType::I1 => {
                                format!(
                                    "  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                                    result_var,
                                    search_father,
                                    search_father,
                                    current_object_ptr,
                                    field_index
                                )
                            }
                            LlvmType::Object => {
                                format!(
                                    "  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                                    result_var,
                                    search_father,
                                    search_father,
                                    current_object_ptr,
                                    field_index
                                )
                            }
                            _ => panic!("Unsupported type for data member access: {}", llvm_type.llvm_type_str()),
                        };
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
                            let load_instr = format!(
                                "  {} = load i8*, i8** {}, align 8\n",
                                load_var,
                                result_var
                            );
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
                        preamble += &format!(
                            "  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 1\n",
                            super_field_ptr,
                            current_type,
                            current_type,
                            current_object_ptr
                        );
                        preamble += &format!(
                            "  {} = load %{}_type*, %{}_type** {}, align 8\n",
                            super_field_load,
                            search_father,
                            search_father,
                            super_field_ptr
                        );
                        current_type = search_father.clone();
                        current_object_ptr = super_field_load;
                    }
                } else {
                    panic!("Member '{}' not found in type '{}'", member_id, current_type);
                }
            }

        };
        
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
        let object_handle = object_result.result_handle.expect("Object for method call must have a result");
        let mut object_ptr_name = object_handle.llvm_name.clone();

        let object_ast_type_name = match &node.obj_type {
            Some(ty) => match ty {
                ast::typing::Type::Defined(defined_type) => defined_type.id.clone(),
                _ => panic!("Object type for method call must be a defined type name"),
            }
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

        let mut call_param_llvm_types_for_sig = vec!["i8*".to_string()];
        let mut call_args_values_with_llvm_types = vec![format!("i8* {}", object_ptr_name)];
        for arg_expr in node.member.arguments.iter_mut() {
            let arg_result = arg_expr.accept(self);
            preamble += &arg_result.preamble;
            let arg_handle = arg_result.result_handle.expect("Function member argument must have a result");
            let arg_llvm_type_str = match arg_expr {
                ast::Expression::NumberLiteral(_) => "double".to_string(),
                ast::Expression::BooleanLiteral(_) => "i1".to_string(),
                ast::Expression::StringLiteral(_) => "i8*".to_string(),
                ast::Expression::Variable(id) => {
                    match &id.info.ty {
                        Some(ty) => match ty {
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => "double".to_string(),
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => "i1".to_string(),
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => "i8*".to_string(),
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => "i8*".to_string(),
                            ast::typing::Type::Defined(name) => format!("%{}_type*", name.id.clone()),
                            _ => "i8*".to_string(),
                        },
                        None => "i8*".to_string(),
                    }
                }
                _ => "i8*".to_string(),
            };
            call_param_llvm_types_for_sig.push(arg_llvm_type_str.clone());
            call_args_values_with_llvm_types.push(format!("{} {}", arg_llvm_type_str, arg_handle.llvm_name));
        }
        let func_signature_ptr_type_for_load = format!("{} ({})*", call_ret_type_str, call_param_llvm_types_for_sig.join(", "));

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
            let super_field_ptr = self.generate_tmp_variable();
            let super_field_load = self.generate_tmp_variable();
            // Try to find the function index in this type's vtable
            if let Some(func_vtable_idx_str) = self.function_member_names.get(&(current_type.clone(), func_name_in_ast.clone())) {
                // Found in this vtable
                
                preamble += &format!(
                    "  {} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}\n",
                    func_ptr_location_in_vtable,
                    vtable_type_name,
                    vtable_type_name,
                    vtable_ptr,
                    func_vtable_idx_str
                );
                let loaded_func_ptr = self.generate_tmp_variable();
                preamble += &format!(
                    "  {} = load {}, {}* {}, align 8\n",
                    loaded_func_ptr, func_signature_ptr_type_for_load, func_signature_ptr_type_for_load, func_ptr_location_in_vtable
                );

                let result_reg: String;
                if call_ret_type_str != "void" {
                    result_reg = self.generate_tmp_variable();
                    preamble += &format!(
                        "  {} = call {} {}({})\n",
                        result_reg, call_ret_type_str, loaded_func_ptr, call_args_values_with_llvm_types.join(", ")
                    );
                } else {
                    result_reg = "".to_string();
                    preamble += &format!(
                        "  call void {}({})\n",
                        loaded_func_ptr, call_args_values_with_llvm_types.join(", ")
                    );
                }

                let result_handle = if call_ret_type_str != "void" {
                    let ast_ret_type = node.member.identifier.info.ty.as_ref().expect("Return type must be known for non-void");
                    Some(LlvmHandle {
                        handle_type: HandleType::Register(self.llvm_type_from_ast_type(ast_ret_type)),
                        llvm_name: result_reg,
                    })
                } else {
                    None
                };

                return VisitorResult {
                    preamble,
                    result_handle,
                };
            } else if let Some(parent_type) = self.inherits.get(&current_type) {
                // Not found, walk up to parent
                // Get super field pointer and load parent object pointer
                
                preamble += &format!(
                    "  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 1\n",
                    super_field_ptr, current_type, current_type, object_typed_ptr
                );
                
                preamble += &format!(
                    "  {} = load %{}_type*, %{}_type** {}, align 8\n",
                    super_field_load, parent_type, parent_type, super_field_ptr
                );
                // Update for next iteration
                current_type = parent_type.clone();
                current_object_ptr = super_field_load;
                // Also update the first argument for the call (self pointer)
                call_args_values_with_llvm_types[0] = format!("i8* {}", current_object_ptr);
            } else {
                panic!("Function member '{}' not found in type '{}' or its ancestors", func_name_in_ast, current_type);
            }
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
            let handle = arg_result.result_handle.expect("Function argument must have a result");
            arg_values.push(handle.llvm_name);

            let arg_type = match arg {
                ast::Expression::NumberLiteral(_) => "double".to_string(),
                ast::Expression::BooleanLiteral(_) => "i1".to_string(),
                ast::Expression::StringLiteral(_) => "i8*".to_string(),
                ast::Expression::Variable(id) => {
                    match &id.info.ty {
                        Some(ty) => match ty {
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => "double".to_string(),
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => "i1".to_string(),
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => "i8*".to_string(),
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => "i8*".to_string(),
                            ast::typing::Type::Defined(name) => format!("%{}_type*", name.id.clone()),
                            _ => "i8*".to_string(),
                        },
                        None => "i8*".to_string(),
                    }
                }
                _ => "i8*".to_string(),
            };
            arg_types.push(arg_type);
        }

        let ret_type = match &node.identifier.info.ty {
            Some(ty) => match ty {
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => "double".to_string(),
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
        todo!()
    }

    fn visit_list_literal(&mut self, node: &mut ast::ListLiteral) -> VisitorResult {
        todo!()
    }

    fn visit_empty_expression(&mut self) -> VisitorResult {
        VisitorResult {
            preamble: "".to_string(),
            result_handle: None,
        }
    }

    fn visit_return_statement(&mut self, node: &mut ast::ReturnStatement) -> VisitorResult {
        let mut preamble = String::new();
        let mut result_handle = None;


        let expr_result = node.expression.accept(self);
        preamble += &expr_result.preamble;
        result_handle = expr_result.result_handle;


        VisitorResult {
            preamble,
            result_handle,
        }
    }

    fn visit_new_expr(&mut self, node: &mut ast::NewExpr) -> VisitorResult {
        let mut preamble = String::new();
        let mut arg_handles = Vec::new();
        let mut arg_types = Vec::new();
        for arg in node.arguments.iter_mut() {
            let arg_result = arg.accept(self);
            preamble += &arg_result.preamble;
            let handle = arg_result.result_handle.expect("Constructor argument must have a result");
            arg_handles.push(handle.llvm_name);
            let arg_type = match arg {
                ast::Expression::NumberLiteral(_) => "double".to_string(),
                ast::Expression::BooleanLiteral(_) => "i1".to_string(),
                ast::Expression::StringLiteral(_) => "i8*".to_string(),
                ast::Expression::Variable(id) => {
                    match &id.info.ty {
                        Some(ty) => match ty {
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => "double".to_string(),
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => "i1".to_string(),
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => "i8*".to_string(),
                            ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => "i8*".to_string(),
                            ast::typing::Type::Defined(name) => format!("%{}_type*", name.id.clone()),
                            _ => "i8*".to_string(),
                        }
                        None => "i8*".to_string(),
                    }
                }
                _ => "i8*".to_string(),
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
            result_handle: Some(crate::llvm_types::LlvmHandle::new_object_register(result_var)),
        }
    }

    fn visist_list_indexing(&mut self, node: &mut ListIndexing) -> VisitorResult {
        todo!()
    }
}

impl DefinitionVisitor<VisitorResult> for GeneratorVisitor {
    fn visit_definition(&mut self, node: &mut Definition) -> VisitorResult {
        node.accept(self)
    }

    fn visit_type_def(&mut self, node: &mut ast::TypeDef) -> VisitorResult {
        let mut preamble = String::new();
        let type_name = &node.name.id;
        let constructor_args: Vec<String> = node.parameter_list.iter().map(|p| self.llvm_type_str_from_ast_type(&p.info.ty.clone().expect("Expected type"))).collect();
            
        self.constructor_args_types.insert(type_name.clone(), constructor_args);

        // --- VTable Type and Global VTable Instance ---
        let vtable_type_name = format!("%{}_vtable_type", type_name);
        let global_vtable_name = format!("@{}_vtable", type_name);

        let mut vtable_fn_ptr_types = Vec::new();
        let mut vtable_initializers = Vec::new();

        preamble += &format!("{} = type {{", vtable_type_name);
        for (i, func_def) in node.function_member_defs.iter().enumerate() {
            let mangled_func_name = format!("{}_{}", type_name, func_def.identifier.id);
            let ret_type_str = match &func_def.identifier.info.ty {
                Some(ty) => self.llvm_type_str_from_ast_type(ty),
                None => "void".to_string(), 
            };
             // for self
            let mut param_llvm_types_for_sig = vec![format!("%{type_name}_type*",).to_string()];
            for param_ast in &func_def.parameters {
                param_llvm_types_for_sig.push(match &param_ast.info.ty {
                    Some(ty) => self.llvm_type_str_from_ast_type(ty),
                    None => "i8*".to_string(),
                });
            }
            let fn_ptr_type_str = format!("{} ({})*", ret_type_str, param_llvm_types_for_sig.join(", "));
            vtable_fn_ptr_types.push(fn_ptr_type_str.clone());
            vtable_initializers.push(format!("{} @{}", fn_ptr_type_str, mangled_func_name));
            self.function_member_names.insert((type_name.clone(), func_def.identifier.id.clone()), i.to_string());
        }
        if vtable_fn_ptr_types.is_empty() {
            preamble += "}\n\n";
            preamble += &format!("{} = private unnamed_addr constant {} {{}}, align 8\n\n", global_vtable_name, vtable_type_name);
        } else {
            preamble += &format!("\n  {}\n", vtable_fn_ptr_types.join(",\n  "));
            preamble += "}\n\n";
            preamble += &format!("{} = private unnamed_addr constant {} {{ {} }}, align 8\n\n",
                                global_vtable_name, vtable_type_name, vtable_initializers.join(", "));
        }
        
        // --- Object Struct Type ---
        preamble += &format!("%{}_type = type {{ \n  {}*,\n", type_name, vtable_type_name);
        let mut field_llvm_types_str = Vec::new();
        
        // Add super field first if we have inheritance
        let has_inheritance = node.inheritance_indicator.is_some();
        let parent_type_name = if has_inheritance {
            let parent = &node.inheritance_indicator.as_ref().unwrap().parent_name.id;
            field_llvm_types_str.push(format!("  %{}_type*", parent));
            // Register super as a member at index 1 (after vtable pointer at index 0)
            self.type_members_ids.insert((type_name.to_string(), "super".to_string()), 1);
            self.inherits.insert(type_name.to_string(), parent.clone());
            Some(parent.clone())
        } else {
            None
        };
        
        // Add regular data members after the super field (if present)
        let member_offset = if has_inheritance { 1 } else { 0 };
        for (i, data_member) in node.data_member_defs.iter().enumerate() {
            let member_llvm_type_str = match data_member.identifier.info.ty.clone() {
                Some(ty) => self.llvm_type_str_from_ast_type(&ty),
                None => {
                    preamble += &format!("  ; WARNING: missing type for member '{}', defaulting to i8*\n", data_member.identifier.id);
                    "i8*".to_string()
                }
            };
            field_llvm_types_str.push(format!("  {}", member_llvm_type_str));
            self.type_members_ids.insert((type_name.to_string(), data_member.identifier.id.clone()), (i + member_offset + 1) as u32);
        }
        if !field_llvm_types_str.is_empty() {
             preamble += &format!("{}\n", field_llvm_types_str.join(",\n"));
        }
        preamble += "}\n\n";

        // --- Constructor (@TypeName_new) ---
        preamble += &format!("define %{}_type* @{}_new(", type_name, type_name);
        let mut ctor_param_defs = Vec::new();
        for (i, param_ast) in node.parameter_list.iter().enumerate() {
            let param_llvm_type = match &param_ast.info.ty {
                Some(ty) => self.llvm_type_str_from_ast_type(ty),
                None => "i8*".to_string(),
            };
            ctor_param_defs.push(format!("{} %{}", param_llvm_type, param_ast.id));
        }
        preamble += &format!("{}) {{\n", ctor_param_defs.join(", "));
        preamble += "entry:\n";
        
        let old_context = std::mem::replace(&mut self.context, Context::new_one_frame());

        for param_ast in &node.parameter_list {
            let param_llvm_type = match &param_ast.info.ty {
                Some(ty) => self.llvm_type_from_ast_type(ty),
                None => LlvmType::Object,
            };
            let param_name = param_ast.id.clone();

            let param_alloca = self.generate_tmp_variable();
            preamble += &self.alloca_statement(&param_alloca, &param_llvm_type);
            preamble += &self.store_statement(&format!("%{}", param_name), &param_alloca, &param_llvm_type);
            
            match param_llvm_type {
                LlvmType::F64 => self.context.define(param_name, Variable::new_f64(param_alloca)),
                LlvmType::I1 => self.context.define(param_name, Variable::new_i1(param_alloca)),
                LlvmType::String => self.context.define(param_name, Variable::new_string(param_alloca)),
                LlvmType::Object => self.context.define(param_name, Variable::new_object(param_alloca)),
            }
        }
        
        let total_fields_count = 1 + node.data_member_defs.len();
        let struct_size_bytes = 8 * total_fields_count;
        let obj_raw_ptr = self.generate_tmp_variable();
        preamble += &format!("  {} = call i8* @malloc(i64 {}) ; Approx size\n", obj_raw_ptr, struct_size_bytes);
        let obj_typed_ptr = self.generate_tmp_variable();
        preamble += &format!("  {} = bitcast i8* {} to %{}_type*\n", obj_typed_ptr, obj_raw_ptr, type_name);

        // Store VTable Pointer
        let vtable_field_ptr = self.generate_tmp_variable();
        preamble += &format!("  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 0\n", vtable_field_ptr, type_name, type_name, obj_typed_ptr);
        preamble += &format!("  store {}* {}, {}** {}, align 8\n", vtable_type_name, global_vtable_name, vtable_type_name, vtable_field_ptr);

        if let Some(inheritance) = &mut node.inheritance_indicator {
            let parent_name = &inheritance.parent_name.id;
            
            let mut arg_preambles = Vec::new();
            let mut arg_handles = Vec::new();
            let mut arg_types = Vec::new();


            arg_types = self.constructor_args_types.get(&parent_name.clone()).unwrap_or_else(|| panic!("Expected args types")).clone();

            for arg in &mut inheritance.argument_list {
                let arg_result = arg.accept(self);
                arg_preambles.push(arg_result.preamble);
                let handle = arg_result.result_handle.expect("Parent constructor arg must not be null");
                arg_handles.push(handle.llvm_name);

            }

            for preamble_str in arg_preambles {
                preamble += &preamble_str;
            }
            
            let call_args = arg_handles
                .iter()
                .zip(arg_types.iter())
                .map(|(a, t)| format!("{} {}", t, a))
                .collect::<Vec<_>>()
                .join(", ");
            
            let super_var = self.generate_tmp_variable();
            preamble += &format!("  {} = call %{}_type* @{}_new({})\n",
                super_var, parent_name, parent_name, call_args);
                
            let super_field_ptr = self.generate_tmp_variable();
            preamble += &format!("  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 1\n", 
                               super_field_ptr, type_name, type_name, obj_typed_ptr);
            
            preamble += &format!("  store %{}_type* {}, %{}_type** {}, align 8\n", 
                               parent_name, super_var, parent_name, super_field_ptr);
        }

        let num_ctor_params = node.parameter_list.len();
        let num_data_members = node.data_member_defs.len();
        let field_offset = if node.inheritance_indicator.is_some() { 2 } else { 1 };
        for i in 0..std::cmp::min(num_ctor_params, num_data_members) {
            let param_ast = &node.parameter_list[i];
            let data_member_ast = &node.data_member_defs[i];

            let field_llvm_type_str = match &data_member_ast.identifier.info.ty {
                 Some(ty) => self.llvm_type_str_from_ast_type(ty),
                 None => "i8*".to_string(),
            };
            let param_llvm_name = format!("%{}", param_ast.id);
            
            let field_ptr = self.generate_tmp_variable();

            preamble += &format!("  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n", 
                                field_ptr, type_name, type_name, obj_typed_ptr, i + field_offset);
            preamble += &format!("  store {} {}, {}* {}, align 8\n", 
                                field_llvm_type_str, param_llvm_name, field_llvm_type_str, field_ptr);
        }
        preamble += &format!("  ret %{}_type* {}\n", type_name, obj_typed_ptr);
        preamble += "}\n\n";
        
        let _ = std::mem::replace(&mut self.context, old_context);

        // --- Method Definitions (@TypeName_methodName) ---
        for func_def_ast in &mut node.function_member_defs {
            let mangled_func_name = format!("{}_{}", type_name, func_def_ast.identifier.id);
            let ret_type_str = match &func_def_ast.identifier.info.ty {
                Some(ty) => self.llvm_type_str_from_ast_type(ty),
                None => "void".to_string(),
            };

            let mut method_param_defs = vec![format!("%{}_type* %self", type_name)];
            for param_ast in &func_def_ast.parameters {
                let param_llvm_type = match &param_ast.info.ty {
                    Some(ty) => self.llvm_type_str_from_ast_type(ty),
                    None => "i8*".to_string(),
                };
                method_param_defs.push(format!("{} %{}", param_llvm_type, param_ast.id));
            }
            preamble += &format!("define {} @{}({}) {{\n", ret_type_str, mangled_func_name, method_param_defs.join(", "));
            preamble += "entry:\n";

            let old_context = std::mem::replace(&mut self.context, Context::new_one_frame());

            let self_alloca = self.generate_tmp_variable();
            preamble += &self.alloca_statement(&self_alloca, &LlvmType::Object);
            preamble += &self.store_statement(&"%self".to_string(), &self_alloca, &LlvmType::Object);
            self.context.define("self".to_string(), Variable::new_object(self_alloca));

            for param_ast in &func_def_ast.parameters {
                let param_name = param_ast.id.clone();
                let ast_param_type = param_ast.info.ty.as_ref().expect("Param type must be known");
                let llvm_param_type_enum = self.llvm_type_from_ast_type(ast_param_type);
                
                let param_alloca = self.generate_tmp_variable();
                preamble += &self.alloca_statement(&param_alloca, &llvm_param_type_enum);
                preamble += &self.store_statement(&format!("%{}", param_name), &param_alloca, &llvm_param_type_enum);
                
                match llvm_param_type_enum {
                    LlvmType::F64 => self.context.define(param_name, Variable::new_f64(param_alloca)),
                    LlvmType::I1 => self.context.define(param_name, Variable::new_i1(param_alloca)),
                    LlvmType::String => self.context.define(param_name, Variable::new_string(param_alloca)),
                    LlvmType::Object => self.context.define(param_name, Variable::new_object(param_alloca)),
                }
            }

            let body_result = match &mut func_def_ast.body {
                ast::FunctionBody::ArrowExpression(arrow_exp) => arrow_exp.expression.accept(self),
                ast::FunctionBody::Block(block) => self.visit_block(block),
            };
            preamble += &body_result.preamble;

            if ret_type_str != "void" {
                if let Some(res_handle) = body_result.result_handle {
                    preamble += &format!("  ret {} {}\n", ret_type_str, res_handle.llvm_name);
                } else {
                    let default_ret_val = match ret_type_str.as_str() {
                        "double" => "0.0",
                        "i1" => "0",
                        _ if ret_type_str.ends_with('*') => "null",
                        _ => panic!("Unexpected return type {}", ret_type_str),
                    };
                    preamble += &format!("  ret {} {}\n", ret_type_str, default_ret_val);
                }
            } else {
                preamble += "  ret void\n";
            }
            preamble += "}\n\n";
            let _ = std::mem::replace(&mut self.context, old_context);
        }
        
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
                ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => "double".to_string(),
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
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => "double".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => "i1".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => "i8*".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => "i8*".to_string(),
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
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => LlvmType::String,
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => LlvmType::Object,
                    ast::typing::Type::Defined(_) => LlvmType::Object,
                    _ => LlvmType::Object,
                },
                None => LlvmType::Object,
            };
            let param_ptr = self.generate_tmp_variable();
            preamble += &self.alloca_statement(&param_ptr, &llvm_type);
            preamble += &self.store_statement(&format!("%{}", param.id), &param_ptr, &llvm_type);
            match llvm_type {
                LlvmType::F64 => self.context.define(param.id.clone(), Variable::new_f64(param_ptr)),
                LlvmType::I1 => self.context.define(param.id.clone(), Variable::new_i1(param_ptr)),
                LlvmType::String => self.context.define(param.id.clone(), Variable::new_string(param_ptr)),
                LlvmType::Object => self.context.define(param.id.clone(), Variable::new_object(param_ptr)),
            };
        }

        let body_result = match &mut node.function_def.body {
            ast::FunctionBody::ArrowExpression(arrow_exp) => {
                let exp_result = arrow_exp.expression.accept(self);
                exp_result
            },
            ast::FunctionBody::Block(block) => {
                let block_result = self.visit_block(block);
                block_result
            }
        };

        preamble += &body_result.preamble;

        if return_type != "void" {
            if let Some(result_handle) = &body_result.result_handle {
                preamble += &format!("  ret {} {}\n", return_type, result_handle.llvm_name);
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
        
        let init_value = init_result.result_handle.expect("Constant initializer must have a result");
        
        match init_value.handle_type.inner_type() {
            LlvmType::F64 => {
                preamble += &format!("@{} = constant double {}\n\n", constant_name, init_value.llvm_name);
            },
            LlvmType::I1 => {
                preamble += &format!("@{} = constant i1 {}\n\n", constant_name, init_value.llvm_name);
            },
            LlvmType::String => {
                if init_value.llvm_name.starts_with("\"") {
                    let string_content = &init_value.llvm_name[1..init_value.llvm_name.len()-1];
                    let byte_length = string_content.len() + 1;
                    
                    preamble += &format!("@{}.str = private constant [{} x i8] c\"{}\\00\", align 1\n", 
                                        constant_name, byte_length, string_content);
                    preamble += &format!("@{} = constant i8* getelementptr inbounds ([{} x i8], [{} x i8]* @{}.str, i64 0, i64 0)\n\n",
                                        constant_name, byte_length, byte_length, constant_name);
                } else {
                    preamble += &format!("@{} = constant i8* {}\n\n", constant_name, init_value.llvm_name);
                }
            },
            LlvmType::Object => {
                preamble += &format!("@{} = constant i8* {}\n\n", constant_name, init_value.llvm_name);
            },
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
            let method_name = &func_sig.identifier.id;
            
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
                format!("{}*", self.llvm_type_str_from_ast_type(inner_type_box.as_ref()))
            }
            _ => panic!("NO implemented type")
        }
    }
}


