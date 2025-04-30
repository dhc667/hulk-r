use super::super::*;
use super::block::ExpressionList;
use super::let_in::Assignment;
use super::*;
use crate::literals::BooleanLiteral;
use crate::tokens::*;
use crate::visitors::Visitor;
use crate::visitors::visitable::Visitable;

pub enum Atom {
    LetIn(LetIn),
    IfElse(IfElse),
    Group(Box<Expression>),
    Print(Print),
    While(While),
    Block(Box<Block>),

    NumberLiteral(NumberLiteral),
    BooleanLiteral(BooleanLiteral),
    Variable(Identifier),
    UnaryOp(UnOp),
}

impl Atom {
    pub fn new_number_literal(start: usize, end: usize, value: &str) -> Self {
        Atom::NumberLiteral(NumberLiteral::new(start, end, value))
    }

    pub fn as_number_literal(&self) -> Option<&NumberLiteral> {
        if let Atom::NumberLiteral(number_literal) = self {
            Some(number_literal)
        } else {
            None
        }
    }

    pub fn new_boolean_literal(start: usize, end: usize, value: bool) -> Self {
        Atom::BooleanLiteral(BooleanLiteral::new(start, end, value))
    }

    pub fn as_boolean_literal(&self) -> Option<&BooleanLiteral> {
        if let Atom::BooleanLiteral(boolean_literal) = self {
            Some(boolean_literal)
        } else {
            None
        }
    }

    pub fn new_identifier(start: usize, end: usize, id: &str) -> Self {
        Atom::Variable(Identifier::new(start, end, id))
    }

    pub fn as_variable(&self) -> Option<&Identifier> {
        if let Atom::Variable(identifier) = self {
            Some(identifier)
        } else {
            None
        }
    }

    pub fn new_grouped_expression(expression: Expression) -> Self {
        Atom::Group(Box::new(expression))
    }

    pub fn as_grouped_expression(&self) -> Option<&Expression> {
        if let Atom::Group(expression) = self {
            Some(expression)
        } else {
            None
        }
    }

    pub fn new_let_expression(
        let_token: Keyword,
        assignments: Vec<Assignment>,
        in_token: Keyword,
        expression: Atom,
    ) -> Self {
        Atom::LetIn(LetIn::new(let_token, assignments, in_token, expression))
    }

    pub fn as_let_expression(&self) -> Option<&LetIn> {
        if let Atom::LetIn(let_expression) = self {
            Some(let_expression)
        } else {
            None
        }
    }

    pub fn new_unary_op(op: UnaryOperator, factor: Atom) -> Self {
        Atom::UnaryOp(UnOp::new(op, factor))
    }

    pub fn as_unary_op(&self) -> Option<&UnOp> {
        if let Atom::UnaryOp(un_op) = self {
            Some(un_op)
        } else {
            None
        }
    }

    pub fn new_if_expression(
        if_token: Keyword,
        condition: Expression,
        then_expression: Atom,
        else_token: Keyword,
        else_expression: Atom,
    ) -> Self {
        Atom::IfElse(IfElse::new(
            if_token,
            condition,
            then_expression,
            else_token,
            else_expression,
        ))
    }

    pub fn as_if_expression(&self) -> Option<&IfElse> {
        if let Atom::IfElse(if_expression) = self {
            Some(if_expression)
        } else {
            None
        }
    }

    pub fn new_while_expression(while_token: Keyword, condition: Expression, body: Atom) -> Self {
        Atom::While(While::new(while_token, condition, body))
    }

    pub fn as_while_expression(&self) -> Option<&While> {
        if let Atom::While(while_expression) = self {
            Some(while_expression)
        } else {
            None
        }
    }

    pub fn new_block(
        open_brace: GroupingOperator,
        expressions: ExpressionList,
        close_brace: GroupingOperator,
    ) -> Self {
        Atom::Block(Box::new(Block::new(open_brace, expressions, close_brace)))
    }

    pub fn as_block(&self) -> Option<&Block> {
        if let Atom::Block(block) = self {
            Some(block)
        } else {
            None
        }
    }

    pub fn new_print_expression(print_token: Keyword, expression: Expression) -> Self {
        Atom::Print(Print {
            print_token,
            expression: Box::new(expression),
        })
    }

    pub fn as_print_expression(&self) -> Option<&Print> {
        if let Atom::Print(print_expression) = self {
            Some(print_expression)
        } else {
            None
        }
    }
}

impl<T: Visitor<R>, R> Visitable<T, R> for Atom {
    fn accept(&mut self, visitor: &mut T) -> R {
        match self {
            Atom::LetIn(let_in) => let_in.accept(visitor),
            Atom::Group(expression) => expression.accept(visitor),
            Atom::IfElse(if_else) => if_else.accept(visitor),
            Atom::Print(print) => print.accept(visitor),
            Atom::While(while_exp) => while_exp.accept(visitor),
            Atom::Block(block) => block.accept(visitor),
            Atom::NumberLiteral(number_literal) => visitor.visit_number_literal(number_literal),
            Atom::Variable(identifier) => visitor.visit_variable(identifier),
            Atom::UnaryOp(un_op) => un_op.accept(visitor),
            Atom::BooleanLiteral(boolean_literal) => visitor.visit_boolean_literal(boolean_literal),
        }
    }
}
