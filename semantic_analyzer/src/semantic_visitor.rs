use ast::{
    VisitableExpression,
    typing::{BuiltInType, Type, TypeAnnotation, to_string},
    *,
};
use generator::context::Context;

use crate::{
    DefinitionInfo,
    typing_utils::{
        get_bin_op_return_type, get_up_op_return_type, is_bin_op_admisible, is_un_op_admisible,
    },
};

pub struct SemanticVisitor {
    pub definitions: Context<DefinitionInfo>,
    pub errors: Vec<String>,
}

impl SemanticVisitor {
    pub fn new() -> Self {
        SemanticVisitor {
            definitions: Context::new_one_frame(),
            errors: Vec::new(),
        }
    }
    fn infer(&self, left: &TypeAnnotation, right: &TypeAnnotation) -> TypeAnnotation {
        // NOTE: this function will change when we add more types
        if left == right { left.clone() } else { None }
    }
}

impl ExpressionVisitor<TypeAnnotation> for SemanticVisitor {
    fn visit_expression(&mut self, node: &mut Expression) -> TypeAnnotation {
        node.accept(self)
    }

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> TypeAnnotation {
        let expr_type = node.expression.accept(self);
        let def_value = self.definitions.get_value(&node.identifier.id);
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
        self.definitions.push_open_frame();

        node.assignment.accept(self);
        let body_type = node.body.accept(self);

        self.definitions.pop_frame();
        body_type
    }

    fn visit_assignment(&mut self, node: &mut Assignment) -> TypeAnnotation {
        let right_type = node.rhs.accept(self);
        self.definitions.define(
            node.identifier.id.clone(),
            DefinitionInfo {
                name: node.identifier.id.clone(),
                is_defined: true,
                position: node.identifier.position.clone(),
                ty: right_type.clone(),
            },
        );
        node.identifier.info.ty = right_type.clone();
        node.identifier.info.definition_pos = Some(node.identifier.position.clone());
        None
    }

    fn visit_if_else(&mut self, node: &mut IfElse) -> TypeAnnotation {
        node.condition.accept(self);
        let then_type = node.then_expression.accept(self);
        let else_type = node.else_expression.accept(self);
        self.infer(&then_type, &else_type)
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
        let def_info = self.definitions.get_value(&node.id);
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
        self.definitions.push_open_frame();

        let iterable_type = node.iterable.accept(self);
        let identifier_type = match &iterable_type {
            Some(Type::Iterable(inner_type)) => Some(*inner_type.clone()),
            _ => None,
        };

        self.definitions.define(
            node.element.id.clone(),
            DefinitionInfo {
                name: node.element.id.clone(),
                is_defined: true,
                position: node.element.position.clone(),
                ty: identifier_type,
            },
        );
        let result = node.body.accept(self);

        self.definitions.pop_frame();
        result
    }

    fn visit_data_member_access(&mut self, node: &mut DataMemberAccess) -> TypeAnnotation {
        todo!()
    }

    fn visit_function_member_access(&mut self, node: &mut FunctionMemberAccess) -> TypeAnnotation {
        todo!()
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
        if node.identifier.id != "print" || node.arguments.len() != 1 {
            todo!();
        }
        node.arguments[0].accept(self)
    }

    fn visit_string_literal(&mut self, _node: &mut StringLiteral) -> TypeAnnotation {
        Some(Type::BuiltIn(BuiltInType::String))
    }

    fn visit_list_literal(&mut self, node: &mut ListLiteral) -> TypeAnnotation {
        let mut result = None;
        let element_types: Vec<_> = node
            .elements
            .iter_mut()
            .map(|item| item.accept(self))
            .collect();
        for elem_type in element_types {
            result = self.infer(&result, &elem_type);
        }
        result
    }

    fn visit_return_statement(&mut self, node: &mut ReturnStatement) -> TypeAnnotation {
        node.expression.accept(self)
    }

    fn visit_block(&mut self, node: &mut Block) -> TypeAnnotation {
        self.definitions.push_open_frame();

        let mut result = None;
        for expression in &mut node.body_items {
            result = expression.accept(self);
        }

        self.definitions.pop_frame();

        result
    }

    fn visit_new_expr(&mut self, node: &mut NewExpr) -> TypeAnnotation {
        todo!()
    }
}
