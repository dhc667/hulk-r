use std::collections::HashMap;

use ast::{
    VisitableExpression,
    typing::{BuiltInType, Type, TypeAnnotation, to_string},
    *,
};
use generator::context::Context;

use crate::{
    def_info::{DefinitionInfo, FuncInfo, TypeInfo, VarInfo}, typing::TypeChecker
};

/// # Description
/// Visitor that performs semantic analysis on the AST.
/// It checks for type correctness, variable definitions, function calls, and other semantic rules.
/// # Arguments
/// * `type_definitions` - A mutable reference to a context that holds the type definitions.
/// * `type_hierarchy` - A reference to a HashMap that holds the inheritance relationship between types.
/// * `var_definitions` - A mutable reference to a context that holds the variable definitions.
/// * `func_definitions` - A mutable reference to a context that holds the function definitions.
/// * `errors` - A mutable reference to a vector that holds the errors encountered during the visit.
/// /// # Note
/// This visitor assumes that the type definitions and variable definitions are already defined in the context.
/// It does not define types or variables, it only checks for their correctness.
pub struct SemanticVisitor<'a> {
    pub type_definitions: &'a mut Context<TypeInfo>,
    pub type_hierarchy: &'a HashMap<String, TypeAnnotation>,
    pub var_definitions: &'a mut Context<VarInfo>,
    pub func_definitions: &'a mut Context<FuncInfo>,
    pub type_checker: TypeChecker,
    pub errors: &'a mut Vec<String>,
}

impl<'a> SemanticVisitor<'a> {
    pub fn new(
        type_definitions: &'a mut Context<TypeInfo>,
        type_hierarchy: &'a HashMap<String, TypeAnnotation>,
        var_definitions: &'a mut Context<VarInfo>,
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
        with_lookup: bool,
    ) -> Option<&DefinitionInfo> {
        let mut current_type = ty.clone();
        loop {
            let Some(ty) = &current_type else { break };

            let type_name = ty.to_string();
            let type_def = self.type_definitions.get_value(&type_name);

            if let Some(type_def) = type_def.and_then(|d| d.as_defined()) {
                if let Some(info) = type_def.members.get(&member_name) {
                    return Some(info);
                }

                // If we are not looking up in the inheritance tree, we can stop here
                if !with_lookup {
                    return None;
                }

                // Try parent type
                let parent_type = self.type_hierarchy.get(&type_name).cloned().expect(&format!(
                   "Type name {} is not found in type tree, this should not happen in semantic visitor",
                    type_name 
                ));
                current_type = parent_type;
                continue;
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

    // Assigments

    fn visit_assignment(&mut self, node: &mut Assignment) -> TypeAnnotation {
        let right_type = node.rhs.accept(self);

        let is_asignable = self
            .type_checker
            .conforms(&right_type, &node.identifier.info.ty);

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
            VarInfo::new_from_identifier(&node.identifier, true, right_type.clone()),
        );
        node.identifier.set_type_if_none(right_type.clone());
        node.identifier.info.definition_pos = Some(node.identifier.position.clone());
        None
    }

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> TypeAnnotation {
        let expr_type = node.rhs.accept(self);
        let assignee_type = node.lhs.accept(self);

        match node.lhs.as_ref() {
            Expression::Variable(variable) => {
                let variable_id = variable.id.clone();
                let def_value = self.var_definitions.get_value(&variable_id);
                match def_value {
                    None => {
                        let message = format!("Variable {} is not defined", variable_id);
                        self.errors.push(message);
                        expr_type
                    }
                    Some(def) if def.is_constant => {
                        let message = format!("Semantic Error: `{}` is not a valid assignment target", variable_id);
        
                        self.errors.push(message);
                        assignee_type
                    }
                    Some(def) if !self.type_checker.conforms(&expr_type, &def.ty) => {
                        let message = format!(
                            "Type mismatch: {} is {} but is being reassigned with {}",
                            variable_id,
                            to_string(&def.ty),
                            to_string(&expr_type)
                        );
                        self.errors.push(message);
                        assignee_type
                    }
                    Some(_) => assignee_type,
                }

            }
            Expression::DataMemberAccess(member) => {
                let member_name = member.member.id.clone();
                if !self.type_checker.conforms(&expr_type, &assignee_type) {
                    let message = format!(
                        "Type mismatch: {} is {} but is being reassigned with {}",
                        member_name,
                        to_string(&member.member.info.ty),
                        to_string(&expr_type)
                    );
                    self.errors.push(message);
                }
                assignee_type
            }
            Expression::ListIndexing(_) => {
                if !self.type_checker.conforms(&expr_type, &assignee_type) {
                    let message = format!(
                        "Type mismatch: Cannot assign {} to list element of type {}",
                        to_string(&expr_type),
                        to_string(&assignee_type)
                    );
                    self.errors.push(message);
                }
                assignee_type
            }
            _ => {
                let message = format!(
                    "Semantic Error: only variables and self properties can be assigned",
                );
                self.errors.push(message);
                None
            }
        }
    }

    // Operators

    fn visit_bin_op(&mut self, node: &mut BinOp) -> TypeAnnotation {
        let left_type = node.lhs.accept(self);
        let right_type = node.rhs.accept(self);

        let op_type = self
            .type_checker
            .check_bin_op(&node.op, &left_type, &right_type, &mut self.errors);
        op_type
    }

    fn visit_un_op(&mut self, node: &mut UnOp) -> TypeAnnotation {

        let operand_type = node.rhs.accept(self);
        let op_type = self
            .type_checker
            .check_up_op(&node.op, &operand_type, &mut self.errors);
        op_type
    }

    // Control flow

    fn visit_let_in(&mut self, node: &mut LetIn) -> TypeAnnotation {
        self.var_definitions.push_open_frame();

        node.assignment.accept(self);
        let body_type = node.body.accept(self);

        self.var_definitions.pop_frame();
        body_type
    }

    fn visit_if_else(&mut self, node: &mut IfElse) -> TypeAnnotation {
        node.condition.accept(self);
        let then_type = node.then_expression.accept(self);
        let else_type = node.else_expression.accept(self);
        self.type_checker
            .get_common_supertype(&then_type, &else_type)
    }

    // Loops

    fn visit_while(&mut self, node: &mut While) -> TypeAnnotation {
        node.condition.accept(self);
        node.body.accept(self)
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
            .conforms(&identifier_type, &node.element.info.ty);

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
            VarInfo::new_from_identifier(&node.element, true, identifier_type.clone()),
        );
        let result = node.body.accept(self);

        node.element.set_type_if_none(identifier_type.clone());
        node.element.info.definition_pos = Some(node.element.position.clone());
        

        self.var_definitions.pop_frame();
        result
    }

    // Literals and Identifiers

    fn visit_variable(&mut self, node: &mut Identifier) -> TypeAnnotation {
        let def_info = self.var_definitions.get_value(&node.id);
        match def_info {
            Some(def) => {
                node.set_type_if_none(def.ty.clone());
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

    // Dot access

    fn visit_data_member_access(&mut self, node: &mut DataMemberAccess) -> TypeAnnotation {
        let member_name = node.member.id.clone();
        let ty = node.object.accept(self);

        // Resolve the member info
        let member_info = self.find_member_info(member_name.clone(), &ty, false);
        let member_info = match member_info.and_then(|d| d.as_var()).cloned() {
            Some(info) => info,
            None => {
                self.errors
                    .push(format!("Could not find data member {}", member_name));
                return None;
            }
        };

        // Annotate identifier
        node.member.set_type_if_none(member_info.ty.clone());

        // Check if expresion is self
        let id_info = node.object.as_variable().and_then(|var| self.var_definitions.get_value(&var.id));
        if let Some(self_info) = id_info {
            if self_info.is_constant {
                return member_info.ty.clone();
            }
        }
        self.errors.push(format!(
            "Cannot access member {} of type {}. Properties are private, even to inherited types.",
            member_name,
            to_string(&ty)
        ));
        member_info.ty.clone()
    }

    fn visit_function_member_access(&mut self, node: &mut FunctionMemberAccess) -> TypeAnnotation {
        let func_name = node.member.identifier.id.clone();
        let ty = node.object.accept(self);

        let func_info = self.find_member_info(func_name.clone(), &ty, true);
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
            node.member.identifier.set_type_if_none(*member_info.get_functor_type().return_type.clone());
            return *member_info.get_functor_type().return_type.clone();
        }

        self.errors
            .push(format!("Could not find method {}", func_name));
        None
    }

    // Other

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
        node.identifier.set_type_if_none(*function_def.get_functor_type().return_type.clone());
        *function_def.get_functor_type().return_type.clone()
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
                VarInfo::new_from_identifier(param, true, None),
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
            VarInfo::new_self_instance(&node.name),
        );

        // Define the data members
        for member in &mut node.data_member_defs {
            let member_type = member.default_value.accept(self);
            
            // Check if type is assignable
            if !self
                .type_checker
                .conforms(&member_type, &member.identifier.info.ty)
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
            member.identifier.set_type_if_none(member_type.clone());

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
                VarInfo::new_from_identifier(&member.identifier, true, member_type),
            );
        }

        //Define the methods
        for method in &mut node.function_member_defs {
            // Define the parameters
            for param in &method.parameters {
                self.var_definitions.define(
                    param.id.clone(),
                    VarInfo::new_from_identifier(param, true, None),
                );
            }

            let method_body_type = method.body.accept(self);

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
            
            if !self.type_checker.conforms(&method_body_type, &method_info.get_functor_type().return_type) {
                self.errors.push(format!(
                    "Type mismatch: Method {} returns {} but {} was found",
                    method.identifier.id,
                    to_string(&method_info.get_functor_type().return_type),
                    to_string(&method_body_type)
                ));
            }

            // annotate the function identifier with return type in the AST
            method.identifier.set_type_if_none(method_body_type.clone());
            // annotate the function identifier with return type in the info
            method_info.name.set_type_if_none(method_body_type.clone());
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
                VarInfo::new_from_identifier(param, true, None),
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
        if !self.type_checker.conforms(&body_type, &func_info.get_functor_type().return_type) {
            self.errors.push(format!(
                "Type mismatch: Function {} returns {} but {} was found",
                node.function_def.identifier.id,
                to_string(&func_info.get_functor_type().return_type),
                to_string(&body_type)
            ));
        }

        node.function_def.identifier.set_type_if_none(body_type.clone());

        // annotate the return type if it is not already annotated
        func_info.name.set_type_if_none(body_type.clone());
        
        
        self.var_definitions.pop_frame();
        None
    }

    fn visit_constant_def(&mut self, node: &mut ConstantDef) -> TypeAnnotation {
        let right_type = node.initializer_expression.accept(self);

        let is_asignable = self
            .type_checker
            .conforms(&right_type, &node.identifier.info.ty);

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
                VarInfo::new_constant_from_identifier(&node.identifier, true, right_type.clone()),
            );
            node.identifier.info.definition_pos = Some(node.identifier.position.clone());
        }
        None
    }

    fn visit_protocol_def(&mut self, _node: &mut ProtocolDef) -> TypeAnnotation {
        todo!()
    }
}
