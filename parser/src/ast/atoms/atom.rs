use crate::tokens::*;
use super::block::ExpressionList;
use super::let_in::Assignment;
use super::*;
use super::super::*;

pub enum Atom {
    LetExpression(LetExpression),
    IfExpression(IfExpression),
    GroupedExpression(Box<Expression>),
    PrintExpression(PrintExpression),
    WhileExpression(WhileExpression),
    Block(Box<Block>),

    NumberLiteral(NumberLiteral),
    Identifier(Identifier),
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

    pub fn new_identifier(start: usize, end: usize, id: &str) -> Self {
        Atom::Identifier(Identifier::new(start, end, id))
    }

    pub fn as_identifier(&self) -> Option<&Identifier> {
        if let Atom::Identifier(identifier) = self {
            Some(identifier)
        } else {
            None
        }
    }

    pub fn new_grouped_expression(expression: Expression) -> Self {
        Atom::GroupedExpression(Box::new(expression))
    }

    pub fn as_grouped_expression(&self) -> Option<&Expression> {
        if let Atom::GroupedExpression(expression) = self {
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
        Atom::LetExpression(LetExpression::new(
            let_token,
            assignments,
            in_token,
            expression,
        ))
    }

    pub fn as_let_expression(&self) -> Option<&LetExpression> {
        if let Atom::LetExpression(let_expression) = self {
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
        Atom::IfExpression(IfExpression::new(
            if_token,
            condition,
            then_expression,
            else_token,
            else_expression,
        ))
    }

    pub fn as_if_expression(&self) -> Option<&IfExpression> {
        if let Atom::IfExpression(if_expression) = self {
            Some(if_expression)
        } else {
            None
        }
    }

    pub fn new_while_expression(while_token: Keyword, condition: Expression, body: Atom) -> Self {
        Atom::WhileExpression(WhileExpression::new(while_token, condition, body))
    }

    pub fn as_while_expression(&self) -> Option<&WhileExpression> {
        if let Atom::WhileExpression(while_expression) = self {
            Some(while_expression)
        } else {
            None
        }
    }

    pub fn new_block(open_brace: GroupingOperator, expressions: ExpressionList, close_brace: GroupingOperator) -> Self {
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
        Atom::PrintExpression(PrintExpression {
            print_token,
            expression: Box::new(expression),
        })
    }

    pub fn as_print_expression(&self) -> Option<&PrintExpression> {
        if let Atom::PrintExpression(print_expression) = self {
            Some(print_expression)
        } else {
            None
        }
    }
}
