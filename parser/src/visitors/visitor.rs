use crate::ast;

pub trait Visitor {
    fn visit_expression(&mut self, node: &mut ast::Expression);

    fn visit_destructive_assignment(&mut self, node: &mut ast::DestructiveAssignment);
    fn visit_bin_op(&mut self, node: &mut ast::BinOp);
    fn visit_atom(&mut self, node: &mut ast::Atom);

    fn visit_let_in(&mut self, node: &mut ast::LetIn);
    fn visit_assignment(&mut self, node: &mut ast::Assignment);

    fn visit_if_else(&mut self, node: &mut ast::IfElse);
    fn visit_print(&mut self, node: &mut ast::Print);
    fn visit_while(&mut self, node: &mut ast::While);
    fn visit_block(&mut self, node: &mut ast::Block);
    fn visit_un_op(&mut self, node: &mut ast::UnOp);
}
