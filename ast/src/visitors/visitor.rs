use crate::{ast::*, tokens::NumberLiteral, variable::Variable};

pub trait Visitor<R> {
    fn visit_program(&mut self, node: &mut Program) -> R;
    fn visit_expression_list(&mut self, node: &mut ExpressionList) -> R;
    fn visit_expression(&mut self, node: &mut Expression) -> R;

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> R;
    fn visit_bin_op(&mut self, node: &mut BinOp) -> R;
    fn visit_atom(&mut self, node: &mut Atom) -> R;

    fn visit_let_in(&mut self, node: &mut LetIn) -> R;
    fn visit_assignment(&mut self, node: &mut Assignment) -> R;

    fn visit_if_else(&mut self, node: &mut IfElse) -> R;
    fn visit_print(&mut self, node: &mut Print) -> R;
    fn visit_while(&mut self, node: &mut While) -> R;
    fn visit_block(&mut self, node: &mut Block) -> R;
    fn visit_un_op(&mut self, node: &mut UnOp) -> R;

    fn visit_variable(&mut self, node: &mut Variable) -> R;
    fn visit_number_literal(&mut self, node: &mut NumberLiteral) -> R;

    fn visit_empty_expression(&mut self) -> R;
}
