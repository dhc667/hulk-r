use parser::{grammar::ProgramParser, visitors::visitable::Visitable};

use crate::GeneratorVisitor;

pub mod lli_interface;

pub mod block;
pub mod dassignment;
pub mod if_else;
pub mod let_in;
pub mod misc;
pub mod operators;
pub mod while_loop;

fn generate_code(hulk: &str) -> String {
    let p = ProgramParser::new();
    let mut ast = p.parse(hulk).unwrap();
    let mut visitor = GeneratorVisitor::new();
    let code = ast.accept(&mut visitor);
    return code.preamble;
}
