use crate::ast::*;

pub trait Visitor {
    fn visit_expression(&mut self, node: &mut Expression);

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment);
    fn visit_bin_op(&mut self, node: &mut BinOp);
    fn visit_atom(&mut self, node: &mut Atom);

    fn visit_let_in(&mut self, node: &mut LetIn);
    fn visit_assignment(&mut self, node: &mut Assignment);

    fn visit_if_else(&mut self, node: &mut IfElse);
    fn visit_print(&mut self, node: &mut Print);
    fn visit_while(&mut self, node: &mut While);
    fn visit_block(&mut self, node: &mut Block);
    fn visit_un_op(&mut self, node: &mut UnOp);
}
