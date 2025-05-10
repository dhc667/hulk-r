use crate::{
    ast::*, tokens::{Identifier, NumberLiteral}, BooleanLiteral, StringLiteral
};

pub trait ExpressionVisitor<R> {
    fn visit_expression(&mut self, node: &mut Expression) -> R;

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> R;
    fn visit_bin_op(&mut self, node: &mut BinOp) -> R;

    fn visit_let_in(&mut self, node: &mut LetIn) -> R;
    fn visit_assignment(&mut self, node: &mut Assignment) -> R;

    fn visit_if_else(&mut self, node: &mut IfElse) -> R;
    fn visit_while(&mut self, node: &mut While) -> R;
    fn visit_for(&mut self, node: &mut For) -> R;
    fn visit_block(&mut self, node: &mut Block) -> R;
    fn visit_return_statement(&mut self, node: &mut ReturnStatement) -> R;
    fn visit_un_op(&mut self, node: &mut UnOp) -> R;

    fn visit_data_member_access(&mut self, node: &mut DataMemberAccess) -> R;
    fn visit_function_member_access(&mut self, node: &mut FunctionMemberAccess) -> R;
    fn visist_list_indexing(&mut self, node: &mut ListIndexing) -> R;

    fn visit_function_call(&mut self, node: &mut FunctionCall) -> R;
    fn visit_variable(&mut self, node: &mut Identifier) -> R;
    fn visit_number_literal(&mut self, node: &mut NumberLiteral) -> R;
    fn visit_boolean_literal(&mut self, node: &mut BooleanLiteral) -> R;
    fn visit_string_literal(&mut self, node: &mut StringLiteral) -> R;
    fn visit_list_literal(&mut self, node: &mut ListLiteral) -> R;

    fn visit_empty_expression(&mut self) -> R;
}
