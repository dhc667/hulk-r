use ast::{
    DefinitionVisitor, Expression, ExpressionVisitor, Identifier, ListLiteral, VisitableDefinition,
    VisitableExpression, token_position::TokenPositionTrait, typing::to_string,
};
use error_handler::error::{
    error::HulkError,
    semantic::type_errors::{NeedsAnAnnotation, NeedsMoreSpecificType, UnknownListType},
};

pub struct TypeVisitor<'a> {
    pub errors: &'a mut Vec<HulkError>,
}

impl<'a> TypeVisitor<'a> {
    pub fn new(errors: &'a mut Vec<HulkError>) -> Self {
        TypeVisitor { errors }
    }

    fn handle_identifier(&mut self, id: &Identifier) {
        if id.info.ty.is_none() {
            self.errors
                .push(NeedsAnAnnotation::new(id.id.clone(), id.position.start).into());
            return;
        }
        if !id.info.ty.as_ref().unwrap().is_specific() {
            self.errors.push(
                NeedsMoreSpecificType::new(
                    id.id.clone(),
                    to_string(&id.info.ty),
                    id.position.start,
                )
                .into(),
            );
        }
    }
    fn handle_list_dec(&mut self, id: &Identifier, list: &mut ListLiteral) {
        if let Some(ty) = id.info.ty.clone() {
            list.list_type = Some(ty);
        }
    }
}

impl<'a> ExpressionVisitor<()> for TypeVisitor<'a> {
    fn visit_expression(&mut self, node: &mut ast::Expression) -> () {
        node.accept(self);
    }

    fn visit_destructive_assignment(&mut self, node: &mut ast::DestructiveAssignment) -> () {
        node.lhs.accept(self);
        node.rhs.accept(self);
    }

    fn visit_bin_op(&mut self, node: &mut ast::BinOp) -> () {
        node.lhs.accept(self);
        node.rhs.accept(self);
    }

    fn visit_let_in(&mut self, node: &mut ast::LetIn) -> () {
        node.assignment.accept(self);
        node.body.accept(self);
    }

    fn visit_new_expr(&mut self, node: &mut ast::NewExpr) -> () {
        node.arguments.iter_mut().for_each(|x| x.accept(self));
    }

    fn visit_assignment(&mut self, node: &mut ast::Assignment) -> () {
        self.handle_identifier(&node.identifier);
        if let Expression::ListLiteral(list) = node.rhs.as_mut() {
            self.handle_list_dec(&node.identifier, list);
        }
        node.rhs.accept(self);
    }

    fn visit_if_else(&mut self, node: &mut ast::IfElse) -> () {
        node.condition.accept(self);
        node.then_expression.accept(self);
        node.else_expression.accept(self);
    }

    fn visit_while(&mut self, node: &mut ast::While) -> () {
        node.condition.accept(self);
        node.body.accept(self);
    }

    fn visit_for(&mut self, node: &mut ast::For) -> () {
        self.handle_identifier(&node.element);
        node.iterable.accept(self);
        node.body.accept(self);
    }

    fn visit_block(&mut self, node: &mut ast::Block) -> () {
        node.body_items
            .iter_mut()
            .for_each(|item| item.accept(self));
    }

    fn visit_return_statement(&mut self, node: &mut ast::ReturnStatement) -> () {
        node.expression.accept(self);
    }

    fn visit_un_op(&mut self, node: &mut ast::UnOp) -> () {
        node.rhs.accept(self);
    }

    fn visit_data_member_access(&mut self, node: &mut ast::DataMemberAccess) -> () {
        node.object.as_mut().accept(self);
    }

    fn visit_function_member_access(&mut self, node: &mut ast::FunctionMemberAccess) -> () {
        node.member.accept(self);
        node.object.as_mut().accept(self);
    }

    fn visist_list_indexing(&mut self, node: &mut ast::ListIndexing) -> () {
        node.index.accept(self);
        node.list.as_mut().accept(self);
    }

    fn visit_function_call(&mut self, node: &mut ast::FunctionCall) -> () {
        node.arguments.iter_mut().for_each(|x| x.accept(self));
    }

    fn visit_variable(&mut self, _node: &mut ast::Identifier) -> () {}

    fn visit_number_literal(&mut self, _node: &mut ast::NumberLiteral) -> () {}

    fn visit_boolean_literal(&mut self, _node: &mut ast::BooleanLiteral) -> () {}

    fn visit_string_literal(&mut self, _node: &mut ast::StringLiteral) -> () {}

    fn visit_list_literal(&mut self, node: &mut ast::ListLiteral) -> () {
        if node.list_type.is_none() {
            self.errors
                .push(UnknownListType::new(node.left_bracket.position()).into());
        }
        node.elements.iter_mut().for_each(|x| x.accept(self));
    }

    fn visit_empty_expression(&mut self) -> () {}
}

impl<'a> DefinitionVisitor<()> for TypeVisitor<'a> {
    fn visit_definition(&mut self, node: &mut ast::Definition) -> () {
        node.accept(self);
    }

    fn visit_type_def(&mut self, node: &mut ast::TypeDef) -> () {
        node.parameter_list
            .iter()
            .for_each(|x| self.handle_identifier(x));

        node.function_member_defs.iter_mut().for_each(|def| {
            def.parameters
                .iter()
                .for_each(|x| self.handle_identifier(x))
        });
        node.data_member_defs.iter_mut().for_each(|x| {
            self.handle_identifier(&x.identifier);
            if let Some(list) = x.default_value.as_list_literal_mut() {
                self.handle_list_dec(&x.identifier, list);
            }
            x.default_value.accept(self);
        });
    }

    fn visit_function_def(&mut self, node: &mut ast::GlobalFunctionDef) -> () {
        node.function_def
            .parameters
            .iter()
            .for_each(|x| self.handle_identifier(x));
    }

    fn visit_constant_def(&mut self, node: &mut ast::ConstantDef) -> () {
        self.handle_identifier(&node.identifier);
        node.initializer_expression.accept(self);
    }

    fn visit_protocol_def(&mut self, _node: &mut ast::ProtocolDef) -> () {
        todo!()
    }
}
