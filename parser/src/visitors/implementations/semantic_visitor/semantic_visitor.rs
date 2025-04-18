use crate::{
    Visitor,
    visitors::{IContext, visitable::Visitable},
};

use super::{DefContext, def_context::Frame};

pub struct SemanticVisitor {
    definitions: DefContext,
}

impl SemanticVisitor {
    pub fn new() -> Self {
        SemanticVisitor {
            definitions: DefContext {
                current_frame: Some(Box::new(Frame::new())),
            },
        }
    }
}

impl Visitor<()> for SemanticVisitor {
    fn visit_expression(&mut self, node: &mut crate::Expression) {
        node.accept(self);
    }

    fn visit_destructive_assignment(&mut self, node: &mut crate::DestructiveAssignment) {
        node.expression.accept(self);
        if !self.definitions.is_defined(&node.identifier.id) {
            panic!("Variable {} is not defined", node.identifier);
        }
    }

    fn visit_bin_op(&mut self, node: &mut crate::BinOp) {
        node.lhs.accept(self);
        node.rhs.accept(self);
    }

    fn visit_atom(&mut self, node: &mut crate::Atom) {
        node.accept(self);
    }

    fn visit_let_in(&mut self, node: &mut crate::LetIn) {
        for assignment in &mut node.assignments {
            self.definitions.push_frame();
            assignment.accept(self);
        }
        node.body.accept(self);
        for _ in &node.assignments {
            self.definitions.pop_frame();
        }
    }

    fn visit_assignment(&mut self, node: &mut crate::Assignment) {
        node.rhs.accept(self);
        if !self.definitions.define(&node.identifier.id) {
            panic!("Variable {} is already defined", node.identifier);
        }
    }

    fn visit_if_else(&mut self, node: &mut crate::IfElse) {
        node.condition.accept(self);
        node.then_expression.accept(self);
        node.else_expression.accept(self);
    }

    fn visit_print(&mut self, node: &mut crate::Print) {
        node.expression.accept(self);
    }

    fn visit_while(&mut self, node: &mut crate::While) -> () {
        node.condition.accept(self);
        node.body.accept(self);
    }

    fn visit_block(&mut self, node: &mut crate::Block) -> () {
        node.expression_list.accept(self);
    }

    fn visit_un_op(&mut self, node: &mut crate::UnOp) -> () {
        node.rhs.accept(self);
    }

    fn visit_variable(&mut self, node: &mut crate::Identifier) -> () {
        if !self.definitions.is_defined(&node.id) {
            panic!("Variable {} is not defined", node.id);
        }
    }

    fn visit_number_literal(&mut self, _node: &mut crate::NumberLiteral) -> () {
        // No action needed for number literals
    }

    fn visit_empty_expression(&mut self) -> () {
        // No action needed for empty expressions
    }

    fn visit_expression_list(&mut self, node: &mut crate::ExpressionList) -> () {
        for expression in &mut node.expressions {
            expression.accept(self);
        }
    }
}
