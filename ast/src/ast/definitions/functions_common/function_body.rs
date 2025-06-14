use crate::{ArrowOperator, Block, Expression, ExpressionVisitor, VisitableExpression};

#[derive(Debug)]
pub enum FunctionBody {
    ArrowExpression(ArrowExpression),
    Block(Block),
}

impl FunctionBody {
    pub fn as_arrow_expression(&self) -> Option<&ArrowExpression> {
        if let Self::ArrowExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_block(&self) -> Option<&Block> {
        if let Self::Block(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<Block> for FunctionBody {
    fn from(v: Block) -> Self {
        Self::Block(v)
    }
}

impl From<ArrowExpression> for FunctionBody {
    fn from(v: ArrowExpression) -> Self {
        Self::ArrowExpression(v)
    }
}

impl<T: ExpressionVisitor<R>, R> VisitableExpression<T, R> for FunctionBody {
    fn accept(&mut self, visitor: &mut T) -> R {
        match self {
            FunctionBody::ArrowExpression(v) => v.expression.accept(visitor),
            FunctionBody::Block(v) => v.accept(visitor),
        }
    }
}

#[derive(Debug)]
pub struct ArrowExpression {
    pub operator: ArrowOperator,
    pub expression: Expression,
}

impl ArrowExpression {
    pub fn new(operator: ArrowOperator, expression: Expression) -> Self {
        Self {
            operator,
            expression,
        }
    }
}
