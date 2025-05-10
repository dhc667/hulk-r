use parser::grammar::ProgramParser;

use crate::CodeGenerator;

pub mod lli_interface;

pub mod block;
mod booleans;
pub mod dassignment;
pub mod if_else;
pub mod let_in;
pub mod misc;
pub mod operators;
pub mod printer;
pub mod while_loop;

fn generate_code(hulk: &str) -> String {
    let p = ProgramParser::new();
    let mut ast = p.parse(hulk).unwrap();
    let code_generator = CodeGenerator::new();

    code_generator.generate_code_from_program_ast(&mut ast)
}
