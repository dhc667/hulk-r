use ast::{BinOp, Expression};

use crate::types::ReturnType;

pub fn reduce_binop(mut v: Vec<ReturnType>) -> ReturnType {
    let r = v.pop().unwrap().try_into_expression().unwrap();
    let op = v.pop().unwrap().try_into_binary_operator().unwrap();
    let l = v.pop().unwrap().try_into_expression().unwrap();

    ReturnType::Expression(Expression::from(BinOp::new(l, op, r)))
}
