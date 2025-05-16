use ast;
use ast::Program;
use ast::tokens;
use ast::visitors::{Visitor, visitable::Visitable};

pub struct EchoVisitor;

impl EchoVisitor {
    pub fn new() -> Self {
        EchoVisitor {}
    }
}

impl Visitor<String> for EchoVisitor {
    fn visit_expression(&mut self, node: &mut ast::Expression) -> String {
        node.accept(self)
    }

    fn visit_destructive_assignment(&mut self, node: &mut ast::DestructiveAssignment) -> String {
        let expression = node.expression.accept(self);
        format!("{} {} {}", node.identifier, node.op, expression)
    }

    fn visit_bin_op(&mut self, node: &mut ast::BinOp) -> String {
        let lhs = node.lhs.accept(self);
        let rhs = node.rhs.accept(self);
        format!("({} {} {})", lhs, node.op, rhs)
    }

    fn visit_let_in(&mut self, node: &mut ast::LetIn) -> String {
        let assignment = node.assignment.accept(self);
        let body = node.body.accept(self);
        format!(
            "{} {} {} {}",
            node.let_token, assignment, node.in_token, body
        )
    }

    fn visit_assignment(&mut self, node: &mut ast::Assignment) -> String {
        let rhs = node.rhs.accept(self);
        format!("{} {} {}", node.identifier, node.op, rhs)
    }

    fn visit_if_else(&mut self, node: &mut ast::IfElse) -> String {
        let condition = node.condition.accept(self);
        let then_branch = node.then_expression.accept(self);
        let else_branch = node.else_expression.accept(self);
        format!(
            "{} ({}) {} {} {}",
            node.if_token, condition, then_branch, node.else_token, else_branch
        )
    }

    fn visit_print(&mut self, node: &mut ast::Print) -> String {
        let expression = node.expression.accept(self);
        format!("{}({})", node.print_token, expression)
    }

    fn visit_while(&mut self, node: &mut ast::While) -> String {
        let condition = node.condition.accept(self);
        let body = node.body.accept(self);
        format!("{} ({}) {}", node.while_token, condition, body)
    }

    fn visit_block(&mut self, node: &mut ast::Block) -> String {
        let inside = node.expression_list.accept(self);
        format!("{} {} {}", node.open_brace, inside, node.close_brace)
    }

    fn visit_un_op(&mut self, node: &mut ast::UnOp) -> String {
        let expression = node.rhs.accept(self);
        format!("({} {})", node.op, expression)
    }

    fn visit_variable(&mut self, node: &mut tokens::Identifier) -> String {
        format!("{}", node)
    }

    fn visit_number_literal(&mut self, node: &mut tokens::NumberLiteral) -> String {
        format!("{}", node)
    }

    fn visit_empty_expression(&mut self) -> String {
        String::new()
    }

    fn visit_expression_list(&mut self, node: &mut ast::ExpressionList) -> String {
        let expressions = node
            .expressions
            .iter_mut()
            .map(|expr| expr.accept(self))
            .collect::<Vec<String>>();

        let result = expressions.join("; ");
        if node.multiple_semicolon_terminated {
            format!("{};;", result)
        } else {
            format!("{};", result)
        }
    }

    fn visit_program(&mut self, node: &mut Program) -> String {
        node.expression_list.accept(self)
    }

    fn visit_boolean_literal(&mut self, node: &mut ast::BooleanLiteral) -> String {
        format!("{}", node)
    }
}
