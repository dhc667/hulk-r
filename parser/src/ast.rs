use std::usize;

pub struct TokenPosition {
    pub start: usize,
    pub end: usize,
}

impl TokenPosition {
    pub fn new(start: usize, end: usize) -> Self {
        TokenPosition {start, end}
    }
}

pub enum Keyword {
    Let(TokenPosition),
    If(TokenPosition),
    Else(TokenPosition),
    While(TokenPosition),
    Print(TokenPosition),
    In(TokenPosition),
    Elif(TokenPosition),
}

pub enum Operator {
    Plus(TokenPosition),
    Minus(TokenPosition),
    Divide(TokenPosition),
    FloorDivide(TokenPosition),
    Times(TokenPosition),
    Modulo(TokenPosition),

    Equal(TokenPosition),
    ColonEqual(TokenPosition),

    OpenBrace(TokenPosition),
    CloseBrace(TokenPosition),
}

pub struct Identifier(pub TokenPosition, pub String);
impl Identifier {
    pub fn new(start: usize, end: usize, id: &str) -> Self {
        Identifier(
            TokenPosition::new(start, end),
            id.to_string()
        )
    }
}

pub struct NumberLiteral(pub TokenPosition, pub f64);
impl NumberLiteral {
    pub fn new(start: usize, end: usize, value: &str) -> Self {
        NumberLiteral(
            TokenPosition::new(start, end),
            value.parse::<f64>().unwrap()
        )
    }
}

pub struct Assignment(pub Identifier, pub Operator, pub Box<Expression>);
impl Assignment {
    pub fn new(identifier: Identifier, op: Operator, expression: Expression) -> Self {
        Assignment(
            identifier,
            op,
            Box::new(expression)
        )
    }
}

pub enum Expression {
    DestructiveAssignment(Identifier, Operator, Box<Expression>),
    Addition(Box<Addition>),
}

impl Expression {
    pub fn new_addition(addition: Addition) -> Self {
        Expression::Addition(
            Box::new(addition)
        )
    }

    pub fn new_destructive_assignment(identifier: Identifier, op: Operator, expression: Expression) -> Self {
        Expression::DestructiveAssignment(
            identifier,
            op,
            Box::new(expression)
        )
    }
}

pub enum Addition {
    BinaryOp(Box<Addition>, Operator, Box<Term>),
    Term(Box<Term>),
}

impl Addition {
    pub fn new_binary_op(left: Addition, op: Operator, right: Term) -> Self {
        Addition::BinaryOp(
            Box::new(left),
            op,
            Box::new(right)
        )
    }
    pub fn new_term(term: Term) -> Self {
        Addition::Term(
            Box::new(term)
        )
    }
}

pub enum Term {
    BinaryOp(Box<Term>, Operator, Box<Factor>),
    Factor(Box<Factor>),
}

impl Term {
    pub fn new_binary_op(left: Term, op: Operator, right: Factor) -> Self {
        Term::BinaryOp(
            Box::new(left),
            op,
            Box::new(right)
        )
    }
    pub fn new_factor(factor: Factor) -> Self {
        Term::Factor(
            Box::new(factor)
        )
    }
}

pub enum Factor {
    BinaryOp(Box<Atom>, Operator, Box<Factor>),
    Atom(Atom),
}

impl Factor {
    pub fn new_binary_op(left: Atom, op: Operator, right: Factor) -> Self {
        Factor::BinaryOp(
            Box::new(left),
            op,
            Box::new(right)
        )
    }
    pub fn new_atom(atom: Atom) -> Self {
        Factor::Atom(
            atom
        )
    }
}

pub enum Atom {
    LetExpression(Keyword, Vec<Assignment>, Keyword, Box<Expression>),
    IfExpression(Vec<ConditionExpressionPair>, Box<Expression>),
    GroupedExpression(Box<Expression>),
    PrintExpression(Keyword, Box<Expression>),
    WhileExpression(Keyword, Box<Expression>, Box<Expression>),
    Block(Box<Block>),

    NumberLiteral(NumberLiteral),
    Identifier(Identifier),
    UnaryOp(Operator, Box<Atom>),
}

impl Atom {
    pub fn new_number_literal(start: usize, end: usize, value: &str) -> Self {
        Atom::NumberLiteral(
            NumberLiteral(
                TokenPosition::new(
                    start,
                    end
                ),
                value.parse::<f64>().unwrap()
            )
        )
    }

    pub fn new_identifier(start: usize, end: usize, id: &str) -> Self {
        Atom::Identifier(
            Identifier(
                TokenPosition::new(
                    start,
                    end
                ),
                id.to_string()
            )
        )
    }

    pub fn new_let_expression(let_token: Keyword, assignments: Vec<Assignment>, in_token: Keyword, expression: Expression) -> Self {
        Atom::LetExpression(
            let_token,
            assignments,
            in_token,
            Box::new(expression)
        )
    }


    pub fn new_unary_op(op: Operator, factor: Atom) -> Self {
        Atom::UnaryOp(
            op,
            Box::new(factor)
        )
    }
}

pub struct Block {
    pub open_brace: Operator,
    pub close_brace: Operator,
    pub expressions: Vec<Expression>,
}

pub struct ConditionExpressionPair {
    pub condition: Box<Expression>,
    pub expression: Box<Expression>,
}
