mod tokens;
use ast::Program;
pub use tokens::*;

mod binops;
pub use binops::*;

mod type_definition;
pub use type_definition::to_type_definition;

use crate::types::{Instruction, ReturnType};

pub fn get_last(mut v: Vec<ReturnType>) -> ReturnType{
    v.pop().unwrap()
}

pub fn program_from_instructions(instructions: Vec<Instruction>) -> Program {
    let mut definitions = Vec::new();
    let mut expressions = Vec::new();

    for instruction in instructions.into_iter() {
        match instruction {
            Instruction::Expression(expr) => expressions.push(expr),
            Instruction::Definition(def) => definitions.push(def),
        }
    }

    return Program::new(definitions, expressions)
}
