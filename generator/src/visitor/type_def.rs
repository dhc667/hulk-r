// This module provides functions to generate LLVM IR types and constructors for user-defined types (classes/structs),
// including support for inheritance, vtables, and method definitions.
//
// The main entry points are:
//   - generate_vtable_type: Generates the LLVM vtable type and global vtable instance for a type, handling inheritance and method overriding.
//   - generate_object_struct_type: Generates the LLVM struct type for the object, including inherited fields.
//   - generate_constructor: Generates the LLVM constructor function for the type, handling field initialization and parent constructor calls.
//   - generate_method_definitions: Generates LLVM function definitions for all methods of the type.

use crate::llvm_types::LlvmType;
use crate::visitor::GeneratorVisitor;
use ast;
use ast::{ExpressionVisitor, VisitableExpression};

/// Generates the LLVM vtable type and global vtable instance for a type.
/// Handles inheritance by copying parent vtable entries and supports method overriding.
///
/// - Adds function pointers for all methods (including inherited and overridden ones) to the vtable.
/// - Maintains mappings in the visitor for method lookup and vtable index.
/// - Returns the LLVM IR string for the vtable type and instance.
pub fn generate_vtable_type(visitor: &mut GeneratorVisitor, node: &mut ast::TypeDef) -> String {
    // The name of the type being processed
    let type_name = &node.name.id;
    // The LLVM type name for the vtable struct
    let vtable_type_name = format!("%{}_vtable_type", type_name);
    // List of function pointer types for the vtable struct
    let mut vtable_fn_ptr_types = Vec::new();
    // List of initializers for the vtable global instance
    let mut vtable_initializers = Vec::new();
    // The string that will accumulate the LLVM IR output
    let mut preamble = String::new();
    preamble += &format!("{} = type {{", vtable_type_name);

    // --- Inheritance: Copy parent vtable methods if any ---
    // If this type inherits from a parent, copy the parent's vtable entries
    if let Some(inheritance) = &node.inheritance_indicator {
        let parent = &inheritance.parent_name.id;
        // Check if the parent has any function member definitions
        if let Some(parent_methods) = visitor.function_member_def_from_type_and_name.iter().filter(|((parent_type, _), _)| parent_type == parent).collect::<Vec<_>>().first() {
            // Collect all parent's methods (name and argument types)
            let parent_methods: Vec<_> = visitor.function_member_def_from_type_and_name.iter()
                .filter(|((parent_type, _), _)| parent_type == parent)
                .map(|((_, method_name), arg_types)| (method_name.clone(), arg_types.clone()))
                .collect();
            for (method_name, arg_types) in parent_methods {
                // Check if the child overrides this method
                let overridden = node.function_member_defs.iter().find(|f| &f.identifier.id == &method_name);
                if let Some(definition) = overridden {
                    // If overridden, insert the child's method into the vtable
                    visitor.function_member_def_from_type_and_name.insert((type_name.clone(), method_name.clone()), arg_types.clone());
                    // Use the child's mangled function name and correct signature
                    let mangled_func_name = format!("{}_{}", type_name, method_name);
                    let ret_type_str = match &definition.identifier.info.ty {
                        Some(ty) => visitor.llvm_type_str_from_ast_type(ty),
                        None => "void".to_string(),
                    };
                    // The first parameter is always a pointer to the type (self)
                    let mut param_llvm_types_for_sig = vec![format!("%{}_type*", type_name)];
                    for param_ast in &definition.parameters {
                        let llvm_type_str = match &param_ast.info.ty {
                            Some(ty) => visitor.llvm_type_str_from_ast_type(ty),
                            None => "i8*".to_string(),
                        };
                        param_llvm_types_for_sig.push(llvm_type_str);
                    }
                    // All vtable entries are stored as i8* for uniformity
                    let fn_ptr_type_str = "i8*";
                    vtable_fn_ptr_types.push(fn_ptr_type_str.to_string());
                    vtable_initializers.push(format!("i8* bitcast ({} ({})* @{} to i8*)", ret_type_str, param_llvm_types_for_sig.join(", "), mangled_func_name));
                    // Map the method to its vtable index
                    visitor.function_member_names.insert(
                        (type_name.clone(), method_name.clone()),
                        (vtable_initializers.len() - 1).to_string()
                    );
                    // Record the original type for this method definition
                    visitor.original_type_for_definition.insert((type_name.clone(), method_name.clone()),type_name.clone());
                }
                else{
                    // If not overridden, copy the parent's vtable entry
                    let original_type_for_def = visitor.original_type_for_definition.get(&(parent.clone(),method_name.clone())).unwrap_or_else(|| panic!("error searching "));
                    visitor.function_member_def_from_type_and_name.insert((type_name.clone(), method_name.clone()), arg_types.clone());
                    // Use the original defining type's mangled function name
                    let mangled_func_name = format!("{}_{}", original_type_for_def, method_name);
                    let ret_type_str = "i8*"; // Use i8* for parent vtable pointer type
                    vtable_fn_ptr_types.push(ret_type_str.to_string());
                    vtable_initializers.push(format!("i8* bitcast (i8* @{} to i8*)", mangled_func_name));
                    visitor.function_member_names.insert(
                        (type_name.clone(), method_name.clone()),
                        (vtable_initializers.len() - 1).to_string()
                    );
                    visitor.original_type_for_definition.insert((type_name.clone(),method_name.clone() ), original_type_for_def.clone());
                }
            }
        }
    }
    // --- End inheritance ---

    // Add this type's own methods to the vtable
    for func_def in node.function_member_defs.iter() {
        // Skip if this method is already in the vtable (i.e., it overrides a parent method and was already handled)
        if visitor.function_member_names.contains_key(&(type_name.clone(), func_def.identifier.id.clone())) {
            continue;
        }
        // Record the original type for this method
        visitor.original_type_for_definition.insert((type_name.clone(), func_def.identifier.id.clone()),type_name.clone());
        let mangled_func_name = format!("{}_{}", type_name, func_def.identifier.id);
        let ret_type_str = match &func_def.identifier.info.ty {
            Some(ty) => visitor.llvm_type_str_from_ast_type(ty),
            None => "void".to_string(),
        };
        // The first parameter is always a pointer to the type (self)
        let mut param_llvm_types_for_sig = vec![format!("%{}_type*", type_name)];
        let mut arg_types = Vec::new();
        for param_ast in &func_def.parameters {
            let llvm_type_str = match &param_ast.info.ty {
                Some(ty) => visitor.llvm_type_str_from_ast_type(ty),
                None => "i8*".to_string(),
            };
            param_llvm_types_for_sig.push(llvm_type_str.clone());
            arg_types.push(llvm_type_str);
        }
        // Map the method signature for later lookup
        visitor.function_member_def_from_type_and_name.insert(
            (type_name.clone(), func_def.identifier.id.clone()),
            arg_types,
        );
        let fn_ptr_type_str = format!("i8*");
        vtable_fn_ptr_types.push(fn_ptr_type_str.clone());
        vtable_initializers.push(format!("i8* bitcast ({} ({})* @{} to i8*)", ret_type_str, param_llvm_types_for_sig.join(", "), mangled_func_name));
        visitor.function_member_names.insert(
            (type_name.clone(), func_def.identifier.id.clone()),
            (vtable_initializers.len() - 1).to_string()
        );
    }

    // Debug: Print function_member_names values for this type
    println!("function_member_names:");
    for ((type_name, method_name), index) in &visitor.function_member_names {
        println!("  (type: {}, method: {}) => vtable index {}", type_name, method_name, index);
    }
    // Emit the vtable struct type
    preamble += &format!("\n  {}\n", vtable_fn_ptr_types.join(",\n  "));
    preamble += "}\n\n";
    // Emit the global vtable instance
    let global_vtable_name = format!("@{}_vtable", type_name);
    preamble += &format!("{} = private unnamed_addr constant {} {{ {} }}, align 8\n\n", global_vtable_name, vtable_type_name, vtable_initializers.join(", "));
    preamble
}

/// Generates the LLVM struct type for the object, including inherited fields.
///
/// - The first field is always a pointer to the vtable.
/// - Inherited fields are included in order, followed by the type's own fields.
/// - Updates the visitor's field index/type mappings for member access.
/// - Returns the LLVM IR string for the struct type.
pub fn generate_object_struct_type(visitor: &mut GeneratorVisitor, node: &mut ast::TypeDef) -> String {
    // The name of the type being processed
    let type_name = &node.name.id;
    // The LLVM type name for the vtable struct
    let vtable_type_name = format!("%{}_vtable_type", type_name);
    // List of LLVM type strings for each field in the struct
    let mut field_llvm_types_str = Vec::new();
    // The index for the next member (starts at 1 because 0 is the vtable pointer)
    let mut member_index = 1;
    
    // --- Inheritance: Copy parent data members if any ---
    // If this type inherits from a parent, copy the parent's data members
    if let Some(inheritance) = &node.inheritance_indicator {
        let parent = &inheritance.parent_name.id;
        // Get all parent's data members from type_members_ids
        let parent_members: Vec<_> = visitor.type_members_ids.iter()
            .filter(|((type_name, _), _)| type_name == parent)
            .collect();
        // Sort parent members by their member index to preserve order
        let mut sorted_parent_members: Vec<_> = parent_members.into_iter()
            .map(|((_, member_name), &index)| (member_name.clone(), index))
            .collect();
        sorted_parent_members.sort_by_key(|(_, index)| *index);
        // For each parent member, add it to the child's fields and maintain index mapping
        for (member_name, parent_index) in sorted_parent_members {
            // NOTE: In a full implementation, the type should be looked up from a registry.
            // Here, we use a placeholder type (double) for demonstration.
            let member_llvm_type_str = "double".to_string();  // Replace with actual type lookup
            // Add to field list
            field_llvm_types_str.push(member_llvm_type_str.clone());
            // Map the member name in child type to the same index it had in parent
            visitor.type_members_ids.insert((type_name.clone(), member_name.clone()), member_index);
            // Increment member index for the child's own members
            member_index += 1;
            // Record the LLVM type for this member in the child
            visitor.type_members_types.insert((type_name.clone(),member_name.clone()),member_llvm_type_str.clone());
        }
    }
    // --- End inheritance ---
    
    // Add this type's own data members
    for data_member in node.data_member_defs.iter() {
        let member_llvm_type_str = match data_member.identifier.info.ty.clone() {
            Some(ty) => visitor.llvm_type_str_from_ast_type(&ty),
            None => "i8*".to_string(),
        };
        field_llvm_types_str.push(member_llvm_type_str.clone());
        visitor.type_members_ids.insert((type_name.clone(), data_member.identifier.id.clone()), member_index);
        member_index += 1;
        visitor.type_members_types.insert((type_name.clone(),data_member.identifier.id.clone()),member_llvm_type_str.clone());
    }
    // Emit the LLVM struct type for the object
    let mut preamble = String::new();
    preamble += &format!("%{}_type = type {{ \n  {}*", type_name, vtable_type_name);
    if !field_llvm_types_str.is_empty() {
        preamble += ",\n";
        preamble += &format!("  {}\n", field_llvm_types_str.join(",\n  "));
    }
    preamble += "}\n\n";
    preamble
}

/// Generates the LLVM constructor function for the type.
///
/// - Allocates memory for the object and sets up the vtable pointer.
/// - Handles parent constructor calls and copies inherited fields.
/// - Initializes the type's own fields from constructor parameters.
/// - Returns the LLVM IR string for the constructor function.
pub fn generate_constructor(visitor: &mut GeneratorVisitor, node: &mut ast::TypeDef) -> String {
    // The name of the type being processed
    let type_name = &node.name.id;
    // The string that will accumulate the LLVM IR output
    let mut preamble = String::new();
    preamble += &format!("define %{}_type* @{}_new(", type_name, type_name);

    // Collect constructor parameter definitions and types
    let mut ctor_param_defs = Vec::new();
    let mut ctor_param_types = Vec::new();
    for param_ast in node.parameter_list.iter() {
        let param_llvm_type = match &param_ast.info.ty {
            Some(ty) => visitor.llvm_type_str_from_ast_type(ty),
            None => "i8*".to_string(),
        };
        ctor_param_defs.push(format!("{} %{}", param_llvm_type, param_ast.id));
        ctor_param_types.push(param_llvm_type);
    }
    // Store constructor argument types for later use (e.g., inheritance)
    visitor.constructor_args_types.insert(type_name.clone(), ctor_param_types);
    preamble += &format!("{}) {{\n", ctor_param_defs.join(", "));
    preamble += "entry:\n";

    // Set up a new context frame for constructor parameters
    let old_context = std::mem::replace(&mut visitor.context, crate::context::Context::new_one_frame());

    // Store constructor parameters in the context for later use
    for param_ast in node.parameter_list.iter() {
        let param_name = param_ast.id.clone();
        let ast_param_type = param_ast.info.ty.as_ref().expect("Param type must be known");
        let llvm_param_type_enum = visitor.llvm_type_from_ast_type(ast_param_type);
        let param_alloca = visitor.generate_tmp_variable();
        preamble += &visitor.alloca_statement(&param_alloca, &llvm_param_type_enum);
        preamble += &visitor.store_statement(&format!("%{}", param_name), &param_alloca, &llvm_param_type_enum);
        // Define the parameter in the context for later lookup
        match llvm_param_type_enum {
            LlvmType::F64 => visitor.context.define(param_name, crate::visitor::Variable::new_f64(param_alloca)),
            LlvmType::I1 => visitor.context.define(param_name, crate::visitor::Variable::new_i1(param_alloca)),
            LlvmType::String => visitor.context.define(param_name, crate::visitor::Variable::new_string(param_alloca)),
            LlvmType::Object => visitor.context.define(param_name, crate::visitor::Variable::new_object(param_alloca)),
        }
    }

    // Collect all type members for this type (including inherited ones)
    let mut field_llvm_types_str: Vec<String> = Vec::new();
    let type_members: Vec<(String, String)> = visitor.type_members_types
        .iter()
        .filter(|((name, _), _)| name == type_name)
        .map(|((_, member_name), member_type)| (member_name.clone(), member_type.clone()))
        .collect();
    for (member_name, member_type) in type_members {
        println!("member_name: {} type {:?}", member_name, member_type);
        field_llvm_types_str.push(member_type);
    }

    // Calculate the approximate struct size (8 bytes for vtable pointer + sum of field sizes)
    let struct_size_bytes = 8 + field_llvm_types_str.iter().map(|t| visitor.llvm_type_size(t)).sum::<usize>();
    // Allocate memory for the object
    let obj_raw_ptr = visitor.generate_tmp_variable();
    preamble += &format!("  {} = call i8* @malloc(i64 {}) ; Approx size\n", obj_raw_ptr, struct_size_bytes);
    // Cast the raw pointer to the typed object pointer
    let obj_typed_ptr = visitor.generate_tmp_variable();
    preamble += &format!("  {} = bitcast i8* {} to %{}_type*\n", obj_typed_ptr, obj_raw_ptr, type_name);

    // Set up the vtable pointer in the object
    let vtable_field_ptr = visitor.generate_tmp_variable();
    let global_vtable_name = format!("@{}_vtable", type_name);
    preamble += &format!("  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 0\n", vtable_field_ptr, type_name, type_name, obj_typed_ptr);
    preamble += &format!("  store %{}_vtable_type* {}, %{}_vtable_type** {}, align 8\n", type_name, global_vtable_name, type_name, vtable_field_ptr);
    
    // --- Inheritance: Handle parent constructor and copy inherited fields ---
    if let Some(ref mut inheritance) = node.inheritance_indicator {
        let parent_type = &inheritance.parent_name.id.clone();
        let parent_ctor_args_expressions = &mut inheritance.argument_list;
        // Get parent constructor argument types
        let parent_ctor_args_types = if let Some(parent_args) = visitor.constructor_args_types.get(parent_type) {
            parent_args.clone()
        } else {
            Vec::new() // No parent constructor arguments
        };
        // Evaluate each parent constructor argument expression and collect their values
        let mut parent_ctor_call_args = Vec::new();
        for (i, expr) in parent_ctor_args_expressions.iter_mut().enumerate() {
            let eval_result = expr.accept(visitor);
            preamble += &eval_result.preamble;
            let parent_arg_type = parent_ctor_args_types.get(i).cloned().unwrap_or_else(|| "i8*".to_string());
            parent_ctor_call_args.push(format!("{} {}", parent_arg_type, eval_result.result_handle.expect("No result handle").llvm_name));
        }
        // Call parent constructor to create parent instance
        let parent_instance = visitor.generate_tmp_variable();
        preamble += &format!("  {} = call %{}_type* @{}_new({})\n",
                             parent_instance, parent_type, parent_type, parent_ctor_call_args.join(", "));
        // Copy parent data members to child object
        // Get sorted parent members by index
        let mut parent_members: Vec<_> = visitor.type_members_ids.iter()
            .filter(|((type_name, _), _)| type_name == parent_type)
            .collect();
        parent_members.sort_by_key(|(_, index)| index.clone());
        for ((parent_type, member_name), &parent_index) in parent_members {
            // Get member type
            println!("Parent type: {} member: {} index: {}", parent_type, member_name, parent_index);
            let member_type = visitor.type_members_types.get(&(parent_type.clone(), member_name.clone()))
                .expect(&format!("Member type for {}.{} not found", parent_type, member_name));
            // Get child member index (should match parent index)
            let child_member_index = visitor.type_members_ids.get(&(type_name.clone(), member_name.clone()))
                .expect(&format!("Child member index for {}.{} not found", type_name, member_name));
            // Get pointer to parent member
            let parent_member_ptr = visitor.generate_tmp_variable();
            preamble += &format!("  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                                 parent_member_ptr, parent_type, parent_type, parent_instance, parent_index);
            // Load parent member value
            let parent_member_value = visitor.generate_tmp_variable();
            preamble += &format!("  {} = load {}, {}* {}, align 8\n",
                                 parent_member_value, member_type, member_type, parent_member_ptr);
            // Get pointer to child member
            let child_member_ptr = visitor.generate_tmp_variable();
            preamble += &format!("  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                                 child_member_ptr, type_name, type_name, obj_typed_ptr, child_member_index);
            // Store parent member value in child
            preamble += &format!("  store {} {}, {}* {}, align 8\n",
                                 member_type, parent_member_value, member_type, child_member_ptr);
        }
    }
    // --- End inheritance ---

    // --- Initialize child's own data members (those not inherited from parent) ---
    // Count the number of parent members to offset the parameter index
    let parent_member_count = if let Some(inheritance) = &node.inheritance_indicator {
        let parent_type = &inheritance.parent_name.id;
        visitor.type_members_ids.iter()
            .filter(|((type_name, _), _)| type_name == parent_type)
            .count()
    } else {
        0
    };
    // For each data member defined in this type, initialize it from the corresponding constructor parameter
    for (i, data_member) in node.data_member_defs.iter().enumerate() {
        let param_index = parent_member_count + i;
        if param_index < node.parameter_list.len() {
            let param_name = &node.parameter_list[param_index].id;
            let param_var = visitor.context.get_value(param_name)
                .expect(&format!("Parameter {} not found in context", param_name));
            // Load parameter value into a register
            let loaded_param = visitor.generate_tmp_variable();
            let (load_preamble, load_handle) = visitor.extract_variable_value_to_register(
                loaded_param.clone(),
                &param_var.llvm_name,
                &param_var.var_type,
            );
            preamble += &load_preamble;
            // Get child member index
            let child_member_index = visitor.type_members_ids.get(&(type_name.clone(), data_member.identifier.id.clone()))
                .expect(&format!("Child member index for {}.{} not found", type_name, data_member.identifier.id));
            // Get pointer to child member
            let child_member_ptr = visitor.generate_tmp_variable();
            preamble += &format!("  {} = getelementptr inbounds %{}_type, %{}_type* {}, i32 0, i32 {}\n",
                                 child_member_ptr, type_name, type_name, obj_typed_ptr, child_member_index);
            // Store parameter value in child member
            let member_type = visitor.type_members_types.get(&(type_name.clone(), data_member.identifier.id.clone()))
                .expect(&format!("Member type for {}.{} not found", type_name, data_member.identifier.id));
            preamble += &format!("  store {} {}, {}* {}, align 8\n",
                                 member_type, load_handle.llvm_name, member_type, child_member_ptr);
        }
    }
    // Restore the previous context frame
    let _ = std::mem::replace(&mut visitor.context, old_context);
    // Return the constructed object pointer
    preamble += &format!("  ret %{}_type* {}\n", type_name, obj_typed_ptr);
    preamble += "}\n\n";
    preamble
}

/// Generates LLVM function definitions for all methods of the type.
///
/// - Each method receives a pointer to the object as the first parameter (self).
/// - Sets up the function body and handles parameter allocation.
/// - Returns the LLVM IR string for all method definitions.
pub fn generate_method_definitions(visitor: &mut GeneratorVisitor, node: &mut ast::TypeDef) -> String {
    // The name of the type being processed
    let type_name = &node.name.id;
    // The string that will accumulate the LLVM IR output
    let mut preamble = String::new();
    // For each method defined in this type
    for func_def_ast in &mut node.function_member_defs {
        // Mangle the function name to include the type name
        let mangled_func_name = format!("{}_{}", type_name, func_def_ast.identifier.id);
        // Determine the LLVM return type string
        let ret_type_str = match &func_def_ast.identifier.info.ty {
            Some(ty) => visitor.llvm_type_str_from_ast_type(ty),
            None => "void".to_string(),
        };
        // The first parameter is always a pointer to the type (self)
        let mut method_param_defs = vec![format!("%{}_type* %self", type_name)];
        // Add all user-defined parameters
        for param_ast in &func_def_ast.parameters {
            let param_llvm_type = match &param_ast.info.ty {
                Some(ty) => visitor.llvm_type_str_from_ast_type(ty),
                None => "i8*".to_string(),
            };
            method_param_defs.push(format!("{} %{}", param_llvm_type, param_ast.id));
        }
        // Emit the function definition header
        preamble += &format!("define {} @{}({}) {{\n", ret_type_str, mangled_func_name, method_param_defs.join(", "));
        preamble += "entry:\n";
        // Set up a new context frame for method parameters
        let old_context = std::mem::replace(&mut visitor.context, crate::context::Context::new_one_frame());
        // Allocate and store the self pointer
        let self_alloca = visitor.generate_tmp_variable();
        preamble += &visitor.alloca_statement(&self_alloca, &LlvmType::Object);
        preamble += &visitor.store_statement(&"%self".to_string(), &self_alloca, &LlvmType::Object);
        visitor.context.define("self".to_string(), crate::visitor::Variable::new_object(self_alloca));
        // Allocate and store all user-defined parameters
        for param_ast in &func_def_ast.parameters {
            let param_name = param_ast.id.clone();
            let ast_param_type = param_ast.info.ty.as_ref().expect("Param type must be known");
            let llvm_param_type_enum = visitor.llvm_type_from_ast_type(ast_param_type);
            let param_alloca = visitor.generate_tmp_variable();
            preamble += &visitor.alloca_statement(&param_alloca, &llvm_param_type_enum);
            preamble += &visitor.store_statement(&format!("%{}", param_name), &param_alloca, &llvm_param_type_enum);
            // Define the parameter in the context for later lookup
            match llvm_param_type_enum {
                LlvmType::F64 => visitor.context.define(param_name, crate::visitor::Variable::new_f64(param_alloca)),
                LlvmType::I1 => visitor.context.define(param_name, crate::visitor::Variable::new_i1(param_alloca)),
                LlvmType::String => visitor.context.define(param_name, crate::visitor::Variable::new_string(param_alloca)),
                LlvmType::Object => visitor.context.define(param_name, crate::visitor::Variable::new_object(param_alloca)),
            }
        }
        // Generate the function body (either an arrow expression or a block)
        let body_result = match &mut func_def_ast.body {
            ast::FunctionBody::ArrowExpression(arrow_exp) => arrow_exp.expression.accept(visitor),
            ast::FunctionBody::Block(block) => visitor.visit_block(block),
        };
        preamble += &body_result.preamble;
        // Emit the return statement
        if ret_type_str != "void" {
            if let Some(res_handle) = body_result.result_handle {
                preamble += &format!("  ret {} {}\n", ret_type_str, res_handle.llvm_name);
            } else {
                // If no return value is produced, emit a default value for the return type
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
        // Restore the previous context frame
        let _ = std::mem::replace(&mut visitor.context, old_context);
    }
    preamble
}
