use super::{
    function_call::FunctionCall, list_indexing::ListIndexing, list_literal::ListLiteral, *,
};
use crate::{ExpressionVisitor, VisitableExpression, tokens::*};

#[derive(Debug)]
pub enum Expression {
    DestructiveAssignment(DestructiveAssignment),
    BinOp(BinOp),

    LetIn(LetIn),
    IfElse(IfElse),
    While(While),
    For(For),
    Block(Box<Block>),

    NumberLiteral(NumberLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
    ListLiteral(ListLiteral),
    NewExpression(NewExpr),

    FunctionCall(FunctionCall),
    DataMemberAccess(DataMemberAccess),
    FunctionMemberAccess(FunctionMemberAccess),
    ListIndexing(ListIndexing),

    Variable(Identifier),
    UnaryOp(UnOp),
}

impl From<NewExpr> for Expression {
    fn from(v: NewExpr) -> Self {
        Self::NewExpression(v)
    }
}

impl From<StringLiteral> for Expression {
    fn from(v: StringLiteral) -> Self {
        Self::StringLiteral(v)
    }
}

impl From<For> for Expression {
    fn from(v: For) -> Self {
        Self::For(v)
    }
}

impl From<ListIndexing> for Expression {
    fn from(v: ListIndexing) -> Self {
        Self::ListIndexing(v)
    }
}

impl From<FunctionMemberAccess> for Expression {
    fn from(v: FunctionMemberAccess) -> Self {
        Self::FunctionMemberAccess(v)
    }
}

impl From<DataMemberAccess> for Expression {
    fn from(v: DataMemberAccess) -> Self {
        Self::DataMemberAccess(v)
    }
}

impl From<DestructiveAssignment> for Expression {
    fn from(v: DestructiveAssignment) -> Self {
        Self::DestructiveAssignment(v)
    }
}

impl From<BinOp> for Expression {
    fn from(v: BinOp) -> Self {
        Self::BinOp(v)
    }
}

impl From<LetIn> for Expression {
    fn from(v: LetIn) -> Self {
        Self::LetIn(v)
    }
}

impl From<IfElse> for Expression {
    fn from(v: IfElse) -> Self {
        Self::IfElse(v)
    }
}

impl From<While> for Expression {
    fn from(v: While) -> Self {
        Self::While(v)
    }
}

impl From<Block> for Expression {
    fn from(v: Block) -> Self {
        Self::Block(Box::new(v))
    }
}

impl From<NumberLiteral> for Expression {
    fn from(v: NumberLiteral) -> Self {
        Self::NumberLiteral(v)
    }
}

impl From<BooleanLiteral> for Expression {
    fn from(v: BooleanLiteral) -> Self {
        Self::BooleanLiteral(v)
    }
}

impl From<ListLiteral> for Expression {
    fn from(v: ListLiteral) -> Self {
        Self::ListLiteral(v)
    }
}

impl From<FunctionCall> for Expression {
    fn from(v: FunctionCall) -> Self {
        Self::FunctionCall(v)
    }
}

impl From<Identifier> for Expression {
    fn from(v: Identifier) -> Self {
        Self::Variable(v)
    }
}

impl From<UnOp> for Expression {
    fn from(v: UnOp) -> Self {
        Self::UnaryOp(v)
    }
}

impl Expression {
    pub fn as_destructive_assignment(&self) -> Option<&DestructiveAssignment> {
        if let Self::DestructiveAssignment(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_bin_op(&self) -> Option<&BinOp> {
        if let Self::BinOp(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_let_in(&self) -> Option<&LetIn> {
        if let Self::LetIn(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_if_else(&self) -> Option<&IfElse> {
        if let Self::IfElse(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_while(&self) -> Option<&While> {
        if let Self::While(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_block(&self) -> Option<&Box<Block>> {
        if let Self::Block(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_number_literal(&self) -> Option<&NumberLiteral> {
        if let Self::NumberLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_boolean_literal(&self) -> Option<&BooleanLiteral> {
        if let Self::BooleanLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_variable(&self) -> Option<&Identifier> {
        if let Self::Variable(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_unary_op(&self) -> Option<&UnOp> {
        if let Self::UnaryOp(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_list_literal(&self) -> Option<&ListLiteral> {
        if let Self::ListLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_list_literal_mut(&mut self) -> Option<&mut ListLiteral> {
        if let Self::ListLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_function_call(&self) -> Option<&FunctionCall> {
        if let Self::FunctionCall(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_data_member_access(&self) -> Option<&DataMemberAccess> {
        if let Self::DataMemberAccess(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_function_member_access(&self) -> Option<&FunctionMemberAccess> {
        if let Self::FunctionMemberAccess(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_list_indexing(&self) -> Option<&ListIndexing> {
        if let Self::ListIndexing(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_string_literal(&self) -> Option<&StringLiteral> {
        if let Self::StringLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_for(&self) -> Option<&For> {
        if let Self::For(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_new_expression(&self) -> Option<&NewExpr> {
        if let Self::NewExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for Expression {
    fn accept(&mut self, visitor: &mut T) -> R {
        match self {
            Expression::LetIn(let_in) => let_in.accept(visitor),
            Expression::IfElse(if_else) => if_else.accept(visitor),
            Expression::While(while_exp) => while_exp.accept(visitor),
            Expression::Block(block) => block.accept(visitor),
            Expression::NumberLiteral(number_literal) => {
                visitor.visit_number_literal(number_literal)
            }
            Expression::Variable(identifier) => visitor.visit_variable(identifier),
            Expression::UnaryOp(un_op) => un_op.accept(visitor),
            Expression::BooleanLiteral(boolean_literal) => {
                visitor.visit_boolean_literal(boolean_literal)
            }
            Expression::DestructiveAssignment(assignment) => assignment.accept(visitor),
            Expression::BinOp(bin_op) => bin_op.accept(visitor),
            Expression::ListLiteral(list_literal) => list_literal.accept(visitor),
            Expression::FunctionCall(function_call) => function_call.accept(visitor),
            Expression::DataMemberAccess(data_member_access) => data_member_access.accept(visitor),
            Expression::FunctionMemberAccess(function_member_access) => {
                function_member_access.accept(visitor)
            }
            Expression::ListIndexing(list_indexing) => list_indexing.accept(visitor),
            Expression::For(for_exp) => for_exp.accept(visitor),
            Expression::StringLiteral(string_literal) => {
                visitor.visit_string_literal(string_literal)
            }
            Expression::NewExpression(new_expr) => visitor.visit_new_expr(new_expr),
        }
    }
}
