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
use ast::{Expression, ExpressionVisitor, VisitableExpression, Definition, DefinitionVisitor, VisitableDefinition, BlockBodyItem};
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
}

impl GeneratorVisitor {
    pub fn new() -> Self {
        GeneratorVisitor {
            context: Context::new_one_frame(),
            tmp_variable_id: 0,
            variable_ids: HashMap::new(),
            type_members_ids: HashMap::new(),
        }
    }

    fn save_type_member_indices_from_defs(&mut self, type_name: &str, data_member_defs: &[ast::DataMemberDef]) {
        for (i, data_member) in data_member_defs.iter().enumerate() {
            let member_id = data_member.identifier.id.clone();
            self.type_members_ids.insert((type_name.to_string(), member_id), i as u32);
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
        
        let type_name = match &object_type {
            Some(ty) => to_string(&Some(ty.clone())),
            
            None => panic!("Object type not found for data member access"),
        };
        let member_id = node.member.id.clone();
        let field_index = *self.type_members_ids.get(&(type_name.clone(), member_id.clone()))
            .unwrap_or_else(|| panic!("Field '{}' not found in type '{}'", member_id, type_name));
        let result_var = self.generate_tmp_variable();

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
        todo!()
    }

    fn visist_list_indexing(&mut self, node: &mut ast::ListIndexing) -> VisitorResult {
        todo!()
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
        VisitorResult {
            preamble: String::new(),
            result_handle: Some(LlvmHandle::new_string_literal(node.string.clone())),
        }
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
}

impl DefinitionVisitor<VisitorResult> for GeneratorVisitor {
    fn visit_definition(&mut self, node: &mut Definition) -> VisitorResult {
        node.accept(self)
    }

    fn visit_type_def(&mut self, node: &mut ast::TypeDef) -> VisitorResult {
        
        let mut preamble = String::new();
        let type_name = &node.name.id;
 
        preamble += &format!("%{}_type = type {{\n", type_name);

        let mut field_types = Vec::new();

        for (i, data_member) in node.data_member_defs.iter().enumerate() {
            let member_type = match data_member.identifier.info.ty.clone() {
                Some(ty) => match ty {
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => "double".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => "i1".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => "i8*".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => "i8*".to_string(),
                    ast::typing::Type::Defined(ref name) => format!("%{}_type*", name.id.clone()),
                    _ => "i8*".to_string(),
                },
                None => {
                    preamble += &format!(
                        "  ; WARNING: missing type for member '{}', defaulting to i8*\n",
                        data_member.identifier.id
                    );
                    "i8*".to_string()
                }
            };

            field_types.push((data_member.identifier.id.clone(), member_type.clone()));
            
            preamble += &format!("  {}", member_type);
            if i < node.data_member_defs.len() - 1 {
                preamble += ",\n";
            } else {
                preamble += "\n";
            }
        }
        
        preamble += "}\n\n";
        
        preamble += &format!("define %{}_type* @{}_new(", type_name, type_name);
        
        for (i, param) in node.parameter_list.iter().enumerate() {
            if i > 0 {
                preamble += ", ";
            }

            let llvm_type = match param.info.ty.clone() {
                Some(ty) => match ty {
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Number) => "double".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Bool) => "i1".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::String) => "i8*".to_string(),
                    ast::typing::Type::BuiltIn(ast::typing::BuiltInType::Object) => "i8*".to_string(),
                    ast::typing::Type::Defined(ref name) => format!("%{}_type*", name.id.clone()),
                    _ => "i8*".to_string(),
                },
                None => {
                    preamble += &format!(
                        "  ; WARNING: missing type for member '{}', defaulting to i8*\n",
                        param.id
                    );
                    "i8*".to_string()
                }
            };
            
            preamble += &format!("{} %{}", llvm_type, param.id);
        }
        
        preamble += ") {\n";
        preamble += "entry:\n";
        
        let struct_ptr = self.generate_tmp_variable();
        let struct_cast = self.generate_tmp_variable();
        
        preamble += &format!("  {} = call i8* @malloc(i64 {})\n", struct_ptr, 8 * field_types.len());
        preamble += &format!("  {} = bitcast i8* {} to %{}_type*\n", struct_cast, struct_ptr, type_name);
        
        let num_params = std::cmp::min(node.parameter_list.len(), field_types.len());
        
        for i in 0..num_params {
            let param = &node.parameter_list[i];
            let (field_name, field_type) = &field_types[i];
            
            let gep_var = self.generate_tmp_variable();
            preamble += &format!(
                "  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                gep_var, type_name, type_name, struct_cast, i
            );
            
            preamble += &format!(
                "  store {} %{}, {}* {}, align 8\n",
                field_type, param.id, field_type, gep_var
            );
        }
        
        preamble += &format!("  ret %{}_type* {}\n", type_name, struct_cast);
        preamble += "}\n\n";
        
        for func_def in &mut node.function_member_defs {
            let func_name = format!("{}_{}", type_name, func_def.identifier.id);
            
            preamble += &format!("define i8* @{}(i8* %self", func_name);
            
            for (i, param) in func_def.parameters.iter().enumerate() {
                preamble += &format!(", double %{}", param.id);
            }
            preamble += ") {\n";
            preamble += "entry:\n";
            preamble += "  ret i8* null\n";
            preamble += "}\n\n";
        }
        
        
        self.save_type_member_indices_from_defs(type_name, &node.data_member_defs);

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
                _ => "i8*".to_string(),
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


