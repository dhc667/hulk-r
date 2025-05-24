use std::collections::HashMap;

use ast::{
    VisitableExpression,
    typing::{BuiltInType, Type, TypeAnnotation, to_string},
    *,
};
use generator::context::Context;

use crate::{
    DefinitionInfo, FuncInfo, GlobalDefinitionInfo, TypeChecker, TypeInfo,
    typing_utils::{
        get_bin_op_return_type, get_up_op_return_type, is_bin_op_admisible, is_un_op_admisible,
    },
};

pub struct SemanticVisitor<'a> {
    pub type_definitions: &'a mut Context<TypeInfo>,
    pub type_hierarchy: &'a HashMap<String, TypeAnnotation>,
    pub var_definitions: &'a mut Context<DefinitionInfo>,
    pub func_definitions: &'a mut Context<FuncInfo>,
    pub type_checker: TypeChecker,
    pub errors: &'a mut Vec<String>,
}

impl<'a> SemanticVisitor<'a> {
    pub fn new(
        type_definitions: &'a mut Context<TypeInfo>,
        type_hierarchy: &'a HashMap<String, TypeAnnotation>,
        var_definitions: &'a mut Context<DefinitionInfo>,
        func_definitions: &'a mut Context<FuncInfo>,
        errors: &'a mut Vec<String>,
    ) -> Self {
        let mut flattened_hierarchy = HashMap::new();
        for type_key in type_hierarchy.keys() {
            let type_info = type_definitions.get_value(type_key).expect(&format!(
                "Type {} is not defined, this should not happen in semantic visitor",
                type_key
            ));
            flattened_hierarchy.insert(type_key.clone(), type_info.get_type_annotation());
        }


        SemanticVisitor {
            type_definitions,
            type_hierarchy,
            var_definitions,
            func_definitions,
            type_checker: TypeChecker::new(type_hierarchy, flattened_hierarchy),
            errors,
        }
    }

    fn find_member_info(
        &self,
        member_name: String,
        ty: &TypeAnnotation,
    ) -> Option<&GlobalDefinitionInfo> {
        let mut current_type = ty.clone();
        loop {
            let Some(ty) = &current_type else { break };

            let type_name = ty.to_string();
            let type_def = self.type_definitions.get_value(&type_name);

            if let Some(type_def) = type_def.and_then(|d| d.as_defined()) {
                if let Some(info) = type_def.members.get(&member_name) {
                    return Some(info);
                }
                // Try parent type
                let parent_type = self.type_hierarchy.get(&type_name).cloned();
                if let Some(parent) = parent_type {
                    current_type = parent;
                    continue;
                }
                // Is not defined in inheritance tree
                panic!(
                    "Type name {} is not found in type tree, this should not happen in semantic visitor",
                    type_name
                );
            }
            current_type = None;
        }
        None
    }
}

impl<'a> ExpressionVisitor<TypeAnnotation> for SemanticVisitor<'a> {
    fn visit_expression(&mut self, node: &mut Expression) -> TypeAnnotation {
        node.accept(self)
    }

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> TypeAnnotation {
        let expr_type = node.expression.accept(self);
        let def_value = self.var_definitions.get_value(&node.identifier.id);
        match def_value {
            Some(def) => {
                if def.ty != expr_type {
                    let message = format!(
                        "Type mismatch: {} is {} but is being reassigned with {}",
                        node.identifier.id,
                        to_string(&def.ty),
                        to_string(&expr_type)
                    );
                    self.errors.push(message);
                }
                def.ty.clone()
            }
            None => {
                let message = format!("Variable {} is not defined", node.identifier.id);
                self.errors.push(message);
                expr_type
            }
        }
    }

    fn visit_bin_op(&mut self, node: &mut BinOp) -> TypeAnnotation {
        let op_type = Some(get_bin_op_return_type(&node.op));

        let left_type = node.lhs.accept(self);
        let right_type = node.rhs.accept(self);

        if !is_bin_op_admisible(&left_type, &node.op) || !is_bin_op_admisible(&right_type, &node.op)
        {
            let message = format!(
                "Type mismatch: Cannot apply {} to operands of type {} and {}",
                node.op,
                to_string(&left_type),
                to_string(&right_type)
            );
            self.errors.push(message)
        }
        op_type
    }

    fn visit_let_in(&mut self, node: &mut LetIn) -> TypeAnnotation {
        self.var_definitions.push_open_frame();

        node.assignment.accept(self);
        let body_type = node.body.accept(self);

        self.var_definitions.pop_frame();
        body_type
    }

    fn visit_assignment(&mut self, node: &mut Assignment) -> TypeAnnotation {
        let right_type = node.rhs.accept(self);

        let is_asignable = self
            .type_checker
            .is_subtype(&right_type, &node.identifier.info.ty);

        if !is_asignable {
            let message = format!(
                "Type mismatch: Cannot assign {} to {}",
                to_string(&right_type),
                to_string(&node.identifier.info.ty)
            );
            self.errors.push(message);
        }

        self.var_definitions.define(
            node.identifier.id.clone(),
            DefinitionInfo::new_from_identifier(&node.identifier, true, right_type.clone()),
        );
        node.identifier.info.ty = right_type.clone();
        node.identifier.info.definition_pos = Some(node.identifier.position.clone());
        None
    }

    fn visit_if_else(&mut self, node: &mut IfElse) -> TypeAnnotation {
        node.condition.accept(self);
        let then_type = node.then_expression.accept(self);
        let else_type = node.else_expression.accept(self);
        self.type_checker
            .get_common_supertype(&then_type, &else_type)
    }

    fn visit_while(&mut self, node: &mut While) -> TypeAnnotation {
        node.condition.accept(self);
        node.body.accept(self)
    }

    fn visit_un_op(&mut self, node: &mut UnOp) -> TypeAnnotation {
        let op_type = Some(get_up_op_return_type(&node.op));

        let operand_type = node.rhs.accept(self);
        if !is_un_op_admisible(&operand_type, &node.op) {
            let message = format!(
                "Type mismatch: Cannot apply {} to operand of type {}",
                node.op,
                to_string(&operand_type)
            );
            self.errors.push(message);
        }
        op_type
    }

    fn visit_variable(&mut self, node: &mut Identifier) -> TypeAnnotation {
        let def_info = self.var_definitions.get_value(&node.id);
        match def_info {
            Some(def) => {
                node.info.ty = def.ty.clone();
                node.info.definition_pos = Some(def.position.clone());
                def.ty.clone()
            }
            None => {
                let message = format!("Variable {} is not defined", node.id);
                self.errors.push(message);
                None
            }
        }
    }

    fn visit_number_literal(&mut self, _node: &mut NumberLiteral) -> TypeAnnotation {
        Some(Type::BuiltIn(BuiltInType::Number))
    }

    fn visit_empty_expression(&mut self) -> TypeAnnotation {
        None
    }

    fn visit_boolean_literal(&mut self, _node: &mut BooleanLiteral) -> TypeAnnotation {
        Some(Type::BuiltIn(BuiltInType::Bool))
    }

    fn visit_for(&mut self, node: &mut For) -> TypeAnnotation {
        self.var_definitions.push_open_frame();

        let iterable_type = node.iterable.accept(self);
        let identifier_type = match &iterable_type {
            Some(Type::Iterable(inner_type)) => Some(*inner_type.clone()),
            _ => None,
        };

        let is_assignable = self
            .type_checker
            .is_subtype(&identifier_type, &node.element.info.ty);

        if !is_assignable {
            let message = format!(
                "Type mismatch: {} is {} but is being assigned {}",
                node.element.id,
                to_string(&identifier_type),
                to_string(&node.element.info.ty)
            );
            self.errors.push(message);
        }

        self.var_definitions.define(
            node.element.id.clone(),
            DefinitionInfo::new_from_identifier(&node.element, true, identifier_type.clone()),
        );
        let result = node.body.accept(self);

        node.element.info.ty = identifier_type.clone();
        

        self.var_definitions.pop_frame();
        result
    }

    fn visit_data_member_access(&mut self, node: &mut DataMemberAccess) -> TypeAnnotation {
        let member_name = node.member.id.clone();
        let ty = node.object.accept(self);

        let member_info = self.find_member_info(member_name.clone(), &ty);
        if let Some(member_info) = member_info.and_then(|d| d.as_var()) {
            node.member.info.ty = member_info.ty.clone();
            return member_info.ty.clone();
        }

        self.errors
            .push(format!("Could not find data member {}", member_name));
        None
    }

    fn visit_function_member_access(&mut self, node: &mut FunctionMemberAccess) -> TypeAnnotation {
        let func_name = node.member.identifier.id.clone();
        let ty = node.object.accept(self);

        let func_info = self.find_member_info(func_name.clone(), &ty);
        if let Some(member_info) = func_info.and_then(|d| d.as_func()) {
            let member_info = member_info.clone();
            let parameter_types: Vec<TypeAnnotation> = node
                .member
                .arguments
                .iter_mut()
                .map(|arg| arg.accept(self))
                .collect();
            let fn_check_result = self
                .type_checker
                .check_functor_call(&member_info, &parameter_types);

            if let Err(errors) = fn_check_result {
                for error in errors {
                    self.errors.push(error);
                }
            }
            node.member.identifier.info.ty = *member_info.functor_type.return_type.clone();
            return *member_info.functor_type.return_type.clone();
        }

        self.errors
            .push(format!("Could not find method {}", func_name));
        None
    }

    fn visist_list_indexing(&mut self, node: &mut ListIndexing) -> TypeAnnotation {
        let iterable_type = node.list.accept(self);
        let member_type = match &iterable_type {
            Some(Type::Iterable(inner_type)) => Some(*inner_type.clone()),
            _ => None,
        };

        let index_type = node.index.accept(self);
        if index_type != Some(Type::BuiltIn(BuiltInType::Number)) {
            let message = format!(
                "Type mismatch: Cannot use index of type {} to access iterable",
                to_string(&index_type)
            );
            self.errors.push(message);
        };
        return member_type;
    }

    fn visit_function_call(&mut self, node: &mut FunctionCall) -> TypeAnnotation {
        if node.identifier.id == "print" {
            if node.arguments.len() != 1 {
                let message = format!(
                    "Type mismatch: print function expects 1 argument, but {} were provided",
                    node.arguments.len()
                );
                self.errors.push(message);
                return None;
            }
            let arg_type = node.arguments[0].accept(self);
            if !vec![
                Some(Type::BuiltIn(BuiltInType::String)),
                Some(Type::BuiltIn(BuiltInType::Number)),
                Some(Type::BuiltIn(BuiltInType::Bool)),
            ]
            .contains(&arg_type)
            {
                let message = format!(
                    "Type mismatch: print function expects argument of type String, but {} was provided",
                    to_string(&arg_type)
                );
                self.errors.push(message);
                return None;
            }
            return arg_type;
        }

        // Check if the function is defined
        let fn_info = self.func_definitions.get_value(&node.identifier.id);

        let Some(function_def) = fn_info else {
            let message = format!("Function {} is not defined", node.identifier.id);
            self.errors.push(message);
            return None;
        };

        let function_def = function_def.clone();

        // Check if parameter types match
        let parameter_types: Vec<TypeAnnotation> = node
            .arguments
            .iter_mut()
            .map(|arg| arg.accept(self))
            .collect();
        let fn_check_result = self
            .type_checker
            .check_functor_call(&function_def, &parameter_types);

        if let Err(errors) = fn_check_result {
            for error in errors {
                self.errors.push(error);
            }
        }
        node.identifier.info.ty = *function_def.functor_type.return_type.clone();
        *function_def.functor_type.return_type.clone()
    }

    fn visit_string_literal(&mut self, _node: &mut StringLiteral) -> TypeAnnotation {
        Some(Type::BuiltIn(BuiltInType::String))
    }

    fn visit_list_literal(&mut self, node: &mut ListLiteral) -> TypeAnnotation {
        let mut result_type = None;
        for item in &mut node.elements {
            let item_type = item.accept(self);
            result_type = self
                .type_checker
                .get_common_supertype(&result_type, &item_type)
        }
        match result_type {
            Some(result_type) => Some(Type::Iterable(Box::new(result_type))),
            None => todo!("We need a way to handle unknown list types"),
        }
    }

    fn visit_return_statement(&mut self, node: &mut ReturnStatement) -> TypeAnnotation {
        node.expression.accept(self)
    }

    fn visit_block(&mut self, node: &mut Block) -> TypeAnnotation {
        self.var_definitions.push_open_frame();

        let mut result = None;
        for expression in &mut node.body_items {
            result = expression.accept(self);
        }

        self.var_definitions.pop_frame();

        result
    }

    fn visit_new_expr(&mut self, node: &mut NewExpr) -> TypeAnnotation {
        let type_def = self.type_definitions
            .get_value(&node.type_name)
            .and_then(|d| d.as_defined())
            .cloned();
        if let Some(type_def) = type_def {
            let mut parameter_types = Vec::new();
            for arg in &mut node.arguments {
                parameter_types.push(arg.accept(self));
            }
            let constructor_check_result = self
                .type_checker
                .check_type_constructor(&type_def, &parameter_types);
            if let Err(errors) = constructor_check_result {
                for error in errors {
                    self.errors.push(error);
                }
            }
            return Some(Type::Defined(type_def.name.clone()));
        }
        self.errors
            .push(format!("Type {} is not defined", &node.type_name));
        None
    }
}

impl<'a> DefinitionVisitor<TypeAnnotation> for SemanticVisitor<'a> {
    fn visit_definition(&mut self, node: &mut Definition) -> TypeAnnotation {
        node.accept(self)
    }

    fn visit_type_def(&mut self, node: &mut TypeDef) -> TypeAnnotation {
        // Define the parameters
        self.var_definitions.push_closed_frame();
        for param in &node.parameter_list {
            self.var_definitions.define(
                param.id.clone(),
                DefinitionInfo::new_from_identifier(param, true, None),
            );
        }

        // Check the super constructor
        if let Some(inheritance) = &mut node.inheritance_indicator {
            let parent_type = self.type_definitions
                .get_value(&inheritance.parent_name.id)
                .and_then(|d| d.as_defined())
                .expect(
                    &format!(
                        "Type {} is not defined, this should not happen in Semantic visitor",
                        inheritance.parent_name.id
                    )
                )
                .clone();

            // Check if the arguments match
            let argument_types: Vec<TypeAnnotation> = inheritance
                .argument_list
                .iter_mut()
                .map(|arg| arg.accept(self))
                .collect();
            let constructor_check_result = self
                .type_checker
                .check_type_constructor(&parent_type, &argument_types);
            if let Err(errors) = constructor_check_result {
                for error in errors {
                    self.errors.push(error);
                }
            }
        }

        // Define Reference to self
        self.var_definitions.define(
            "self".to_string(),
            DefinitionInfo::new(
                node.name.id.clone(),
                true,
                node.name.position,
                Some(Type::Defined(node.name.clone())),
            ),
        );

        // Define the data members
        for member in &mut node.data_member_defs {
            let member_type = member.default_value.accept(self);

            // Check if type is assignable
            if !self
                .type_checker
                .is_subtype(&member_type, &member.identifier.info.ty)
            {
                let message = format!(
                    "Type mismatch: Member {} is {} but is being assigned {}",
                    member.identifier.id,
                    to_string(&member.identifier.info.ty),
                    to_string(&member_type)
                );
                self.errors.push(message);
            }
            // anotate the type of the member
            member.identifier.info.ty = member_type.clone();

            let member_info = self.type_definitions
                .get_value_mut(&node.name.id)
                .and_then(|d| d.as_defined_mut())
                .expect(&format!(
                    "Type {} is not defined, this should not happen in Semantic visitor",
                    &node.name.id
                ))
                .members
                .get_mut(&member.identifier.id)
                .and_then(|d| d.as_var_mut())
                .expect(&format!(
                    "Member {} is not defined in type {}, this should not happen in Semantic visitor",
                    &member.identifier.id, 
                    &node.name.id
                ));

            if member_info.ty.is_none() {
                member_info.ty = member_type.clone();
            }

            self.var_definitions.define(
                member.identifier.id.clone(),
                DefinitionInfo::new_from_identifier(&member.identifier, true, member_type),
            );
        }

        //Define the methods
        for method in &mut node.function_member_defs {
            // Define the parameters
            for param in &method.parameters {
                self.var_definitions.define(
                    param.id.clone(),
                    DefinitionInfo::new_from_identifier(param, true, None),
                );
            }

            let method_type = method.body.accept(self);
            self.var_definitions.define(
                method.identifier.id.clone(),
                DefinitionInfo::new_from_identifier(&method.identifier, true, method_type.clone()),
            );

            // Check if type is assignable to return type
            let type_info = self
                .type_definitions
                .get_value_mut(&node.name.id)
                .and_then(|d| d.as_defined_mut())
                .expect(&format!(
                    "Type {} is not defined, this should not happen in Semantic visitor",
                    &node.name.id
                ));

            let method_info = type_info
                .members
                .get_mut(&method.identifier.id)
                .and_then(|d| d.as_func_mut())
                .expect(&format!(
                    "Method {} is not defined in type {}, this should not happen in Semantic visitor",
                    &method.identifier.id, 
                    &node.name.id
                ));
            
            if !self.type_checker.is_subtype(&method_type, &method_info.functor_type.return_type) {
                self.errors.push(format!(
                    "Type mismatch: Method {} returns {} but {} was found",
                    method.identifier.id,
                    to_string(&method_info.functor_type.return_type),
                    to_string(&method_type)
                ));
            }

            method.identifier.info.ty = method_type.clone();

            // annotate the return type if it is not already annotated
            if method_info.functor_type.return_type.is_none() {
                method_info.functor_type.return_type = Box::new(method_type);
            }
        }
        
        self.var_definitions.pop_frame();        
        None
    }

    fn visit_function_def(&mut self, node: &mut GlobalFunctionDef) -> TypeAnnotation {
        self.var_definitions.push_closed_frame();

        // Define the parameters
        for param in &node.function_def.parameters {
            self.var_definitions.define(
                param.id.clone(),
                DefinitionInfo::new_from_identifier(param, true, None),
            );
        }

        let body_type = node.function_def.body.accept(self);
        let func_info = self.func_definitions
                .get_value_mut(&node.function_def.identifier.id)
                .expect(&format!(
                    "Function {} is not defined, this should not happen in Semantic visitor",
                    &node.function_def.identifier.id, 
                ));
        // Check if type is assignable to return type
        if !self.type_checker.is_subtype(&body_type, &func_info.functor_type.return_type) {
            self.errors.push(format!(
                "Type mismatch: Function {} returns {} but {} was found",
                node.function_def.identifier.id,
                to_string(&func_info.functor_type.return_type),
                to_string(&body_type)
            ));
        }

        node.function_def.identifier.info.ty = body_type.clone();

        // annotate the return type if it is not already annotated
        if func_info.functor_type.return_type.is_none() {
            func_info.functor_type.return_type = Box::new(body_type.clone());
        }
        
        self.var_definitions.pop_frame();
        None
    }

    fn visit_constant_def(&mut self, node: &mut ConstantDef) -> TypeAnnotation {
        let right_type = node.initializer_expression.accept(self);

        let is_asignable = self
            .type_checker
            .is_subtype(&right_type, &node.identifier.info.ty);

        if !is_asignable {
            let message = format!(
                "Type mismatch: Cannot assign {} to {}",
                to_string(&right_type),
                to_string(&node.identifier.info.ty)
            );
            self.errors.push(message);
        }

        if self.var_definitions.is_defined(&node.identifier.id) {
            let message = format!(
                "Constant {} is already defined",
                node.identifier.id
            );
            self.errors.push(message);
        } else {
            self.var_definitions.define(
                node.identifier.id.clone(),
                DefinitionInfo::new_from_identifier(&node.identifier, true, right_type.clone()),
            );
            node.identifier.info.definition_pos = Some(node.identifier.position.clone());
        }
        None
    }

    fn visit_protocol_def(&mut self, _node: &mut ProtocolDef) -> TypeAnnotation {
        todo!()
    }
}
