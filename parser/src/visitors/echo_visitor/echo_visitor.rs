use ast::{Expression, tokens};
use ast::{ExpressionVisitor, VisitableExpression};

pub struct EchoVisitor;

impl EchoVisitor {
    pub fn new() -> Self {
        EchoVisitor {}
    }

    fn parenthesise_accessed_object_if_necessary(&mut self, object: &mut Expression) -> String {
        match object {
            ast::Expression::DestructiveAssignment(_)
            | ast::Expression::BinOp(_)
            | ast::Expression::LetIn(_)
            | ast::Expression::IfElse(_)
            | ast::Expression::While(_)
            | ast::Expression::For(_)
            | ast::Expression::NewExpression(_)
            | ast::Expression::UnaryOp(_) => {
                format!("({})", object.accept(self))
            }
            ast::Expression::Block(_)
            | ast::Expression::NumberLiteral(_)
            | ast::Expression::BooleanLiteral(_)
            | ast::Expression::StringLiteral(_)
            | ast::Expression::ListLiteral(_)
            | ast::Expression::FunctionCall(_)
            | ast::Expression::DataMemberAccess(_)
            | ast::Expression::FunctionMemberAccess(_)
            | ast::Expression::ListIndexing(_)
            | ast::Expression::Variable(_) => object.accept(self),
        }
    }
}

impl ExpressionVisitor<String> for EchoVisitor {
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

    fn visit_while(&mut self, node: &mut ast::While) -> String {
        let condition = node.condition.accept(self);
        let body = node.body.accept(self);
        format!("{} ({}) {}", node.while_token, condition, body)
    }

    fn visit_for(&mut self, node: &mut ast::For) -> String {
        format!(
            "{}({} {} {}){}",
            node.for_token,
            node.element.to_string(),
            node.in_token,
            node.iterable.accept(self),
            node.body.accept(self)
        )
    }

    fn visit_block(&mut self, node: &mut ast::Block) -> String {
        let expressions = node
            .body_items
            .iter_mut()
            .map(|expr| expr.accept(self))
            .collect::<Vec<String>>();

        let result = expressions.join("; ");

        let inside = if node.multiple_semicolon_terminated {
            format!("{};;", result)
        } else {
            format!("{};", result)
        };

        format!("{} {} {}", node.open_brace, inside, node.close_brace)
    }

    fn visit_un_op(&mut self, node: &mut ast::UnOp) -> String {
        let expression = node.rhs.accept(self);
        format!("({} {})", node.op, expression)
    }

    fn visit_data_member_access(&mut self, node: &mut ast::DataMemberAccess) -> String {
        let accessee = self.parenthesise_accessed_object_if_necessary(node.object.as_mut());

        format!("{}{}{}", accessee, node.op, node.member.to_string())
    }

    fn visit_function_member_access(&mut self, node: &mut ast::FunctionMemberAccess) -> String {
        let accessee = self.parenthesise_accessed_object_if_necessary(node.object.as_mut());

        format!("{}{}{}", accessee, node.op, node.member.accept(self))
    }

    fn visist_list_indexing(&mut self, node: &mut ast::ListIndexing) -> String {
        let accessee = self.parenthesise_accessed_object_if_necessary(node.list.as_mut());

        format!(
            "{}{}{}{}",
            accessee,
            node.open_brace,
            node.index.accept(self),
            node.close_brace
        )
    }

    fn visit_function_call(&mut self, node: &mut ast::FunctionCall) -> String {
        format!(
            "{}({})",
            node.identifier.to_string(),
            node.arguments
                .iter_mut()
                .map(|e| e.accept(self))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn visit_variable(&mut self, node: &mut tokens::Identifier) -> String {
        format!("{}", node)
    }

    fn visit_number_literal(&mut self, node: &mut tokens::NumberLiteral) -> String {
        format!("{}", node)
    }

    fn visit_boolean_literal(&mut self, node: &mut ast::BooleanLiteral) -> String {
        format!("{}", node)
    }

    fn visit_string_literal(&mut self, node: &mut ast::StringLiteral) -> String {
        format!("\"{}\"", node.string)
    }

    fn visit_list_literal(&mut self, node: &mut ast::ListLiteral) -> String {
        format!(
            "[{}]",
            node.elements
                .iter_mut()
                .map(|e| e.accept(self))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn visit_empty_expression(&mut self) -> String {
        String::new()
    }

    fn visit_return_statement(&mut self, node: &mut ast::ReturnStatement) -> String {
        format!("{} {}", node.return_token, node.expression.accept(self))
    }

    fn visit_new_expr(&mut self, node: &mut ast::NewExpr) -> String {
        format!(
            "{} {}({})",
            node.new_token,
            node.type_name,
            node.arguments
                .iter_mut()
                .map(|arg| arg.accept(self))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
