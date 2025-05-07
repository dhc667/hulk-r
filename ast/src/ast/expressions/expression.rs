use super::*;
use crate::tokens::*;
use crate::visitors::Visitor;
use crate::visitors::visitable::Visitable;

pub enum Expression {
    DestructiveAssignment(DestructiveAssignment),
    BinOp(BinOp),

    LetIn(LetIn),
    IfElse(IfElse),
    Print(Print),
    While(While),
    Block(Box<Block>),

    NumberLiteral(NumberLiteral),
    BooleanLiteral(BooleanLiteral),
    Variable(Identifier),
    UnaryOp(UnOp),
}

impl Expression {
    pub fn new_number_literal(start: usize, end: usize, value: &str) -> Self {
        Expression::NumberLiteral(NumberLiteral::new(start, end, value))
    }

    pub fn as_number_literal(&self) -> Option<&NumberLiteral> {
        if let Expression::NumberLiteral(number_literal) = self {
            Some(number_literal)
        } else {
            None
        }
    }

    pub fn new_boolean_literal(start: usize, end: usize, value: bool) -> Self {
        Expression::BooleanLiteral(BooleanLiteral::new(start, end, value))
    }

    pub fn as_boolean_literal(&self) -> Option<&BooleanLiteral> {
        if let Expression::BooleanLiteral(boolean_literal) = self {
            Some(boolean_literal)
        } else {
            None
        }
    }

    pub fn new_identifier(start: usize, end: usize, id: &str) -> Self {
        Expression::Variable(Identifier::new(start, end, id))
    }

    pub fn as_variable(&self) -> Option<&Identifier> {
        if let Expression::Variable(identifier) = self {
            Some(identifier)
        } else {
            None
        }
    }

    pub fn new_let_expression(
        let_token: Keyword,
        assignments: Vec<Assignment>,
        in_token: Keyword,
        expression: Expression,
    ) -> Self {
        Expression::LetIn(LetIn::new(let_token, assignments, in_token, expression))
    }

    pub fn as_let_expression(&self) -> Option<&LetIn> {
        if let Expression::LetIn(let_expression) = self {
            Some(let_expression)
        } else {
            None
        }
    }

    pub fn new_unary_op(op: UnaryOperator, factor: Expression) -> Self {
        Expression::UnaryOp(UnOp::new(op, factor))
    }

    pub fn as_unary_op(&self) -> Option<&UnOp> {
        if let Expression::UnaryOp(un_op) = self {
            Some(un_op)
        } else {
            None
        }
    }

    pub fn new_if_expression(
        if_token: Keyword,
        condition: Expression,
        then_expression: Expression,
        else_token: Keyword,
        else_expression: Expression,
    ) -> Self {
        Expression::IfElse(IfElse::new(
            if_token,
            condition,
            then_expression,
            else_token,
            else_expression,
        ))
    }

    pub fn as_if_expression(&self) -> Option<&IfElse> {
        if let Expression::IfElse(if_expression) = self {
            Some(if_expression)
        } else {
            None
        }
    }

    pub fn new_while_expression(while_token: Keyword, condition: Expression, body: Expression) -> Self {
        Expression::While(While::new(while_token, condition, body))
    }

    pub fn as_while_expression(&self) -> Option<&While> {
        if let Expression::While(while_expression) = self {
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
        Expression::Block(Box::new(Block::new(open_brace, expressions, close_brace)))
    }

    pub fn as_block(&self) -> Option<&Block> {
        if let Expression::Block(block) = self {
            Some(block)
        } else {
            None
        }
    }

    pub fn new_print_expression(print_token: Keyword, expression: Expression) -> Self {
        Expression::Print(Print {
            print_token,
            expression: Box::new(expression),
        })
    }

    pub fn as_print_expression(&self) -> Option<&Print> {
        if let Expression::Print(print_expression) = self {
            Some(print_expression)
        } else {
            None
        }
    }

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
}

impl<T: Visitor<R>, R> Visitable<T, R> for Expression {
    fn accept(&mut self, visitor: &mut T) -> R {
        match self {
            Expression::LetIn(let_in) => let_in.accept(visitor),
            Expression::IfElse(if_else) => if_else.accept(visitor),
            Expression::Print(print) => print.accept(visitor),
            Expression::While(while_exp) => while_exp.accept(visitor),
            Expression::Block(block) => block.accept(visitor),
            Expression::NumberLiteral(number_literal) => visitor.visit_number_literal(number_literal),
            Expression::Variable(identifier) => visitor.visit_variable(identifier),
            Expression::UnaryOp(un_op) => un_op.accept(visitor),
            Expression::BooleanLiteral(boolean_literal) => visitor.visit_boolean_literal(boolean_literal),

            Expression::DestructiveAssignment(assignment) => assignment.accept(visitor),
            Expression::BinOp(bin_op) => bin_op.accept(visitor),
        }
    }
}
