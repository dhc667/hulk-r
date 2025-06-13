use parser::grammar::ProgramParser;
use semantic_analyzer::semantic_analyzer::SemanticAnalyzer;

use crate::CodeGenerator;

pub mod lli_interface;

pub mod block;
mod booleans;
pub mod dassignment;
pub mod global_definition;
pub mod if_else;
pub mod let_in;
pub mod misc;
pub mod operators;
pub mod printer;
pub mod while_loop;

fn generate_code(hulk: &str) -> String {
    let p = ProgramParser::new();
    let mut ast = p.parse(hulk).unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let analysis_result = semantic_analyzer.analyze_program_ast(&mut ast);

    if let Err(errors) = analysis_result {
        for error in errors {
            println!("Error: {}", error);
        }
    } else {
        println!("Semantic analysis successful.");
    }

    let code_generator = CodeGenerator::new();

    code_generator.generate_code_from_program_ast(&mut ast)
}
