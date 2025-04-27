use ast::{BinaryOperator, UnaryOperator, Visitable, Visitor};

use crate::{
    BuiltInType, DefContext, Type,
    hulk_type::{TypeAnnotation, convert_to_string},
};

pub struct TypeCheckerVisitor {
    pub def_context: DefContext,
    pub errors: Vec<String>,
}

impl TypeCheckerVisitor {
    pub fn new(def_context: DefContext) -> Self {
        TypeCheckerVisitor {
            def_context,
            errors: Vec::new(),
        }
    }

    fn infer(&self, left: &TypeAnnotation, right: &TypeAnnotation) -> TypeAnnotation {
        // NOTE: this function will change when we add more types
        if left == right { left.clone() } else { None }
    }

    fn get_bin_op_return_type(op: &BinaryOperator) -> Type {
        match op {
            BinaryOperator::Plus(_) => Type::BuiltIn(BuiltInType::Number),
            BinaryOperator::Minus(_) => Type::BuiltIn(BuiltInType::Number),
            BinaryOperator::Divide(_) => Type::BuiltIn(BuiltInType::Number),
            BinaryOperator::FloorDivide(_) => Type::BuiltIn(BuiltInType::Number),
            BinaryOperator::Times(_) => Type::BuiltIn(BuiltInType::Number),
            BinaryOperator::Modulo(_) => Type::BuiltIn(BuiltInType::Number),
            BinaryOperator::EqualEqual(_) => Type::BuiltIn(BuiltInType::Bool),
            BinaryOperator::Less(_) => Type::BuiltIn(BuiltInType::Bool),
            BinaryOperator::LessEqual(_) => Type::BuiltIn(BuiltInType::Bool),
            BinaryOperator::Greater(_) => Type::BuiltIn(BuiltInType::Bool),
            BinaryOperator::GreaterEqual(_) => Type::BuiltIn(BuiltInType::Bool),
            BinaryOperator::NotEqual(_) => Type::BuiltIn(BuiltInType::Bool),
            BinaryOperator::Or(_) => Type::BuiltIn(BuiltInType::Bool),
            BinaryOperator::And(_) => Type::BuiltIn(BuiltInType::Bool),
            BinaryOperator::ColonEqual(_) => Type::BuiltIn(BuiltInType::Bool),
            BinaryOperator::Equal(_) => todo!(),
        }
    }

    fn get_up_op_return_type(op: &UnaryOperator) -> Type {
        match op {
            UnaryOperator::Plus(_) => Type::BuiltIn(BuiltInType::Number),
            UnaryOperator::Minus(_) => Type::BuiltIn(BuiltInType::Number),
        }
    }

    fn get_bin_op_param_type(op: &BinaryOperator) -> TypeAnnotation {
        match op {
            BinaryOperator::Plus(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::Minus(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::Divide(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::FloorDivide(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::Times(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::Modulo(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::EqualEqual(_) => None,
            BinaryOperator::Less(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::LessEqual(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::Greater(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::GreaterEqual(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            BinaryOperator::NotEqual(_) => None,
            BinaryOperator::Or(_) => Some(Type::BuiltIn(BuiltInType::Bool)),
            BinaryOperator::And(_) => Some(Type::BuiltIn(BuiltInType::Bool)),
            BinaryOperator::ColonEqual(_) => None,
            BinaryOperator::Equal(_) => None,
        }
    }

    fn get_up_op_param_type(op: &UnaryOperator) -> TypeAnnotation {
        match op {
            UnaryOperator::Plus(_) => Some(Type::BuiltIn(BuiltInType::Number)),
            UnaryOperator::Minus(_) => Some(Type::BuiltIn(BuiltInType::Number)),
        }
    }

    fn is_bin_op_admisible(operand_type: &TypeAnnotation, op: &BinaryOperator) -> bool {
        match operand_type {
            None => true,
            Some(ty) => {
                let param_type = TypeCheckerVisitor::get_bin_op_param_type(op);
                match param_type {
                    None => true,
                    Some(param_type) => param_type == *ty,
                }
            }
        }
    }

    fn is_un_op_admisible(operand_type: &TypeAnnotation, op: &UnaryOperator) -> bool {
        match operand_type {
            None => true,
            Some(ty) => {
                let param_type = TypeCheckerVisitor::get_up_op_param_type(op);
                match param_type {
                    None => true,
                    Some(param_type) => param_type == *ty,
                }
            }
        }
    }
}

impl Visitor<TypeAnnotation> for TypeCheckerVisitor {
    fn visit_program(&mut self, node: &mut ast::Program) -> TypeAnnotation {
        node.expression_list.accept(self)
    }

    fn visit_expression_list(&mut self, node: &mut ast::ExpressionList) -> TypeAnnotation {
        let mut result = None;
        for expression in &mut node.expressions {
            result = expression.accept(self);
        }
        result
    }

    fn visit_expression(&mut self, node: &mut ast::Expression) -> TypeAnnotation {
        node.accept(self)
    }

    fn visit_destructive_assignment(
        &mut self,
        node: &mut ast::DestructiveAssignment,
    ) -> TypeAnnotation {
        let var_type = self
            .def_context
            .get_type(&node.identifier.id, node.identifier.context_id.unwrap());
        let expr_type = node.expression.accept(self);
        if var_type != expr_type {
            let message = format!(
                "Type mismatch: {} is {} but is being reassigned with {}",
                node.identifier.id,
                convert_to_string(&var_type),
                convert_to_string(&expr_type)
            );
            self.errors.push(message);
        }
        expr_type
    }

    fn visit_bin_op(&mut self, node: &mut ast::BinOp) -> TypeAnnotation {
        let op_type = Some(TypeCheckerVisitor::get_bin_op_return_type(&node.op));

        let left_type = node.lhs.accept(self);
        let right_type = node.rhs.accept(self);

        if !TypeCheckerVisitor::is_bin_op_admisible(&left_type, &node.op)
            || !TypeCheckerVisitor::is_bin_op_admisible(&right_type, &node.op)
        {
            let message = format!(
                "Type mismatch: Cannot apply {} to operands of type {} and {}",
                node.op,
                convert_to_string(&left_type),
                convert_to_string(&right_type)
            );
            self.errors.push(message)
        }

        op_type
    }

    fn visit_atom(&mut self, node: &mut ast::Atom) -> TypeAnnotation {
        node.accept(self)
    }

    fn visit_let_in(&mut self, node: &mut ast::LetIn) -> TypeAnnotation {
        node.assignment.accept(self);
        node.body.accept(self)
    }

    fn visit_assignment(&mut self, node: &mut ast::Assignment) -> TypeAnnotation {
        let right_type = node.rhs.accept(self);
        self.def_context.set_type(
            &node.identifier.id,
            node.identifier.context_id.unwrap(),
            right_type.clone(),
        );
        right_type
    }

    fn visit_if_else(&mut self, node: &mut ast::IfElse) -> TypeAnnotation {
        node.condition.accept(self);
        let then_type = node.then_expression.accept(self);
        let else_type = node.else_expression.accept(self);
        self.infer(&then_type, &else_type)
    }

    fn visit_print(&mut self, node: &mut ast::Print) -> TypeAnnotation {
        node.expression.accept(self)
    }

    fn visit_while(&mut self, node: &mut ast::While) -> TypeAnnotation {
        node.condition.accept(self);
        node.body.accept(self)
    }

    fn visit_block(&mut self, node: &mut ast::Block) -> TypeAnnotation {
        node.expression_list.accept(self)
    }

    fn visit_un_op(&mut self, node: &mut ast::UnOp) -> TypeAnnotation {
        let op_type = Some(TypeCheckerVisitor::get_up_op_return_type(&node.op));

        let operand_type = node.rhs.accept(self);
        if !TypeCheckerVisitor::is_un_op_admisible(&operand_type, &node.op) {
            let message = format!(
                "Type mismatch: Cannot apply {} to operand of type {}",
                node.op,
                convert_to_string(&operand_type)
            );
            self.errors.push(message);
        }

        op_type
    }

    fn visit_variable(&mut self, node: &mut ast::Identifier) -> TypeAnnotation {
        self.def_context
            .get_type(&node.id, node.context_id.unwrap())
    }

    fn visit_number_literal(&mut self, _node: &mut ast::NumberLiteral) -> TypeAnnotation {
        Some(Type::BuiltIn(BuiltInType::Number))
    }

    fn visit_empty_expression(&mut self) -> TypeAnnotation {
        None
    }

    fn visit_boolean_literal(&mut self, _node: &mut ast::BooleanLiteral) -> TypeAnnotation {
        Some(Type::BuiltIn(BuiltInType::Bool))
    }
}
