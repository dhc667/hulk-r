use ast::{visitors::visitable::Visitable, *};

use super::DefContext;

pub struct SemanticVisitor {
    pub definitions: DefContext,
    pub errors: Vec<String>,
}

impl SemanticVisitor {
    pub fn new() -> Self {
        SemanticVisitor {
            definitions: DefContext::new(),
            errors: Vec::new(),
        }
    }
}

impl Visitor<()> for SemanticVisitor {
    fn visit_expression(&mut self, node: &mut Expression) {
        node.accept(self);
    }

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) {
        node.expression.accept(self);
        let context = self.definitions.get_context(&node.identifier.id);
        match context {
            Some(index) => {
                node.identifier.context_id = Some(index);
            }
            None => {
                let message = format!("Variable {} is not defined", node.identifier);
                self.errors.push(message);
            }
        }
    }

    fn visit_bin_op(&mut self, node: &mut BinOp) {
        node.lhs.accept(self);
        node.rhs.accept(self);
    }

    fn visit_atom(&mut self, node: &mut Atom) {
        node.accept(self);
    }

    fn visit_let_in(&mut self, node: &mut LetIn) {
        self.definitions.push_frame();

        node.assignment.accept(self);
        node.body.accept(self);

        self.definitions.pop_frame();
    }

    fn visit_assignment(&mut self, node: &mut Assignment) {
        node.rhs.accept(self);
        let context = self.definitions.define(&node.identifier.id);
        match context {
            Some(index) => {
                node.identifier.context_id = Some(index);
            }

            None => {
                let message = format!("Variable {} is already defined", node.identifier);
                self.errors.push(message);
            }
        }
    }

    fn visit_if_else(&mut self, node: &mut IfElse) {
        node.condition.accept(self);
        node.then_expression.accept(self);
        node.else_expression.accept(self);
    }

    fn visit_print(&mut self, node: &mut Print) {
        node.expression.accept(self);
    }

    fn visit_while(&mut self, node: &mut While) -> () {
        node.condition.accept(self);
        node.body.accept(self);
    }

    fn visit_block(&mut self, node: &mut Block) -> () {
        node.expression_list.accept(self);
    }

    fn visit_un_op(&mut self, node: &mut UnOp) -> () {
        node.rhs.accept(self);
    }

    fn visit_variable(&mut self, node: &mut Identifier) -> () {
        let context = self.definitions.get_context(&node.id);
        match context {
            Some(index) => {
                node.context_id = Some(index);
            }
            None => {
                let message = format!("Variable {} is not defined", node.id);
                self.errors.push(message);
            }
        }
    }

    fn visit_number_literal(&mut self, _node: &mut NumberLiteral) -> () {
        // No action needed for number literals
    }

    fn visit_empty_expression(&mut self) -> () {
        // No action needed for empty expressions
    }

    fn visit_expression_list(&mut self, node: &mut ExpressionList) -> () {
        for expression in &mut node.expressions {
            expression.accept(self);
        }
    }

    fn visit_program(&mut self, node: &mut Program) -> () {
        node.expression_list.accept(self);
    }
}
