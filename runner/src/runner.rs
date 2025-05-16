use ast::Visitable;
use parser::ProgramParser;
use semantic_analyzer::SemanticVisitor;
use generator::GeneratorVisitor;


fn write_output(target_file: &str, content: &str) -> Result<(), std::io::Error> {
    std::fs::write(target_file, content)?;
    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file = "script.hulk";
    let output_file = "script.ll";

    let content = std::fs::read_to_string(file)?;

    let p = ProgramParser::new();
    let mut ast = p.parse(&content).map_err(|e| format!("Parse error: {}", e))?;
    let mut semantic_visitor = SemanticVisitor::new();
    
    ast.accept(&mut semantic_visitor);
    if !semantic_visitor.errors.is_empty() {
        for error in semantic_visitor.errors {
            println!("Error: {}", error);
        }
        return Err("Semantic errors found".into());
    }

    let mut generator_visitor = GeneratorVisitor::new();

    let generated_code = ast.accept(&mut generator_visitor).preamble;

    write_output(output_file, &generated_code)?;
    Ok(())
}
