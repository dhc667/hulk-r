//! This file is temporarily used to test the code generation.


use generator::GeneratorVisitor;
use parser::{ProgramParser, Visitable};

fn main() {
    let llvm = generate_code("let x = 3 in print(2 + 2 + 3 + 4 * 7 + x);");
    println!("{}", llvm);

    println!("{}", call_lli(&llvm).unwrap())
}

fn generate_code(hulk: &str) -> String{
    let p = ProgramParser::new();
    let mut ast = p.parse(hulk).unwrap();
    let mut visitor = GeneratorVisitor::new();
    let code = ast.accept(&mut visitor);
    return code.preamble;
}

pub fn call_lli(program_str: &str) -> Result<String, String> {
    use std::process::Command;

    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("echo '{}' | lli", program_str))
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "Error executing lli: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let result = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    Ok(result)
}

pub fn lli_f64(program_str: &str) -> Result<f64, String> {
    let value = call_lli(program_str)?;

    match value.parse::<f64>() {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    }
}
