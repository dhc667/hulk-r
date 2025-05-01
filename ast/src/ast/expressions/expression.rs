use super::super::atoms::Atom;
use super::*;
use crate::tokens::*;
use crate::visitors::Visitor;
use crate::visitors::visitable::Visitable;

pub enum Expression {
    DestructiveAssignment(DestructiveAssignment),
    BinOp(BinOp),
    Atom(Box<Atom>),
}

impl Expression {
    pub fn new_destructive_assignment(
        identifier: Identifier,
        op: BinaryOperator,
        rhs: Expression,
    ) -> Self {
        Expression::DestructiveAssignment(DestructiveAssignment::new(identifier, op, rhs))
    }

    pub fn as_destructive_assignment(&self) -> Option<&DestructiveAssignment> {
        if let Expression::DestructiveAssignment(assignment) = self {
            Some(assignment)
        } else {
            None
        }
    }

    pub fn new_binary_op(lhs: Expression, op: BinaryOperator, rhs: Expression) -> Self {
        Expression::BinOp(BinOp::new(lhs, op, rhs))
    }

    pub fn as_bin_op(&self) -> Option<&BinOp> {
        if let Expression::BinOp(bin_op) = self {
            Some(bin_op)
        } else {
            None
        }
    }

    pub fn new_atom(atom: Atom) -> Self {
        Expression::Atom(Box::new(atom))
    }

    pub fn as_atom(&self) -> Option<&Atom> {
        if let Expression::Atom(atom) = self {
            Some(atom)
        } else {
            None
        }
    }
}

impl<T: Visitor<R>, R> Visitable<T, R> for Expression {
    fn accept(&mut self, visitor: &mut T) -> R {
        match self {
            Expression::DestructiveAssignment(assignment) => assignment.accept(visitor),
            Expression::BinOp(bin_op) => bin_op.accept(visitor),
            Expression::Atom(atom) => atom.accept(visitor),
        }
    }
}
