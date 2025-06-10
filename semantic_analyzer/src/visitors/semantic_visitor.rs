mod var_definition;
mod destructive_assignment;
mod function_call;
mod function_def;
mod print;
mod find_member_info;
mod find_method_info;
mod get_conformable;

use std::collections::HashMap;

use ast::{
    VisitableExpression,
    typing::{BuiltInType, Type, TypeAnnotation, to_string},
    *,
};
use generator::context::Context;

use crate::{
    def_info::{FuncInfo, TypeInfo, VarInfo}, typing::TypeChecker
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
/// # Note
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
}

impl<'a> ExpressionVisitor<TypeAnnotation> for SemanticVisitor<'a> {
    fn visit_expression(&mut self, node: &mut Expression) -> TypeAnnotation {
        node.accept(self)
    }

    // Assigments

    fn visit_assignment(&mut self, node: &mut Assignment) -> TypeAnnotation {
        let right_type = node.rhs.accept(self);
        self.handle_var_definition(&mut node.identifier, right_type, true)
    }

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> TypeAnnotation {
        let expr_type = node.rhs.accept(self);
        let assignee_type = node.lhs.accept(self);

        match node.lhs.as_ref() {
            Expression::Variable(variable) => {
                self.handle_reassign_var(variable, &assignee_type, &expr_type)
            }
            Expression::DataMemberAccess(member) => {
                self.handle_field_reassign(&member.member, &assignee_type, &expr_type)
            }
            Expression::ListIndexing(_) => {
                self.handle_list_element_reassign(&assignee_type, &expr_type)
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
            Some(Type::Iterable(inner_type)) => Some(inner_type.as_ref().clone()),
            ty => {
                self.errors.push(
                    format!(
                        "Semantic Error: Cannot iterate over type {}",
                        to_string(&ty)
                    )
                );
                None
            },
        };

        self.var_definitions.define(
            node.element.id.clone(),
            VarInfo::new_from_identifier(&node.element, true, iterable_type.clone()),
        );

        self.handle_var_definition(&mut node.element, identifier_type, true);
        let result = node.body.accept(self);
        
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
        let member_info = self.find_member_info(member_name.clone(), &ty);
        let Some(member_info) = member_info.cloned() else {
            self.errors.push(format!("Could not find data member {}", member_name));
            return None;
        };

        // Annotate identifier
        node.member.set_type_if_none(member_info.ty.clone());
        // Annotate expr
        node.obj_type = ty.clone();

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

        // annotate object
        node.obj_type = ty.clone();

        let func_info = self.find_method_info(func_name.clone(), &ty);
        let Some(func_info) = func_info else {
            self.errors.push(format!("Could not find method {}", func_name));
            return None;
        };
        return self.handle_function_call(func_info.clone(), &mut node.member.identifier, &mut node.member.arguments)
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
            return self.handle_print(&mut node.arguments);
        }
        // Check if the function is defined
        let function_def = self.func_definitions.get_value(&node.identifier.id).cloned();
        let Some(fn_info) = function_def else {
            let message = format!("Function {} is not defined", node.identifier.id);
            self.errors.push(message);
            return None;
        };
        self.handle_function_call(fn_info, &mut node.identifier, &mut node.arguments)
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
            
            self.handle_var_definition(
                &mut member.identifier,
                member_type.clone(),
                true,
            );

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
        }

        //Define the methods
        for method in &mut node.function_member_defs {
            self.var_definitions.push_open_frame();
            self.handle_fn_def(method, Some(&node.name));
            self.var_definitions.pop_frame();
        }
        
        self.var_definitions.pop_frame();        
        None
    }

    fn visit_function_def(&mut self, node: &mut GlobalFunctionDef) -> TypeAnnotation {
        self.var_definitions.push_closed_frame();
        self.handle_fn_def(&mut node.function_def, None);
        self.var_definitions.pop_frame();
        None
    }

    fn visit_constant_def(&mut self, node: &mut ConstantDef) -> TypeAnnotation {
        let right_type = node.initializer_expression.accept(self);
        self.handle_var_definition(&mut node.identifier, right_type, false)
    }

    fn visit_protocol_def(&mut self, _node: &mut ProtocolDef) -> TypeAnnotation {
        todo!()
    }
}
