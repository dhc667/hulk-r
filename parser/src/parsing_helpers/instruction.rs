use ast::{Definition, Expression, Program};

pub enum Instruction {
    Expression(Expression),
    Definition(Definition)
}

impl From<Definition> for Instruction {
    fn from(v: Definition) -> Self {
        Self::Definition(v)
    }
}

impl From<Expression> for Instruction {
    fn from(v: Expression) -> Self {
        Self::Expression(v)
    }
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
