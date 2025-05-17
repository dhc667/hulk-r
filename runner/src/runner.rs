use generator::CodeGenerator;
use parser::ProgramParser;
use semantic_analyzer::semantic_analyzer::SemanticAnalyzer;

fn write_output(target_file: &str, content: &str) -> Result<(), std::io::Error> {
    std::fs::write(target_file, content)?;
    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file = "script.hulk";
    let output_file = "script.ll";

    let content = std::fs::read_to_string(file)?;

    //  NOTE: the parser -> semantic analyzer -> generator steps will eventually
    //        be abstracted away into a single struct or function

    let p = ProgramParser::new();
    let mut ast = p
        .parse(&content)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let analysis_result = semantic_analyzer.analyze_program_ast(&mut ast);
    if let Err(errors) = analysis_result {
        for error in errors {
            println!("Error: {}", error);
        }
        return Err("Semantic errors found".into());
    }

    let code_generator = CodeGenerator::new();
    let generated_code = code_generator.generate_code_from_program_ast(&mut ast);

    write_output(output_file, &generated_code)?;
    Ok(())
}
