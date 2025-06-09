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
    let result =
        String::from_utf8(output.stdout.trim_ascii().to_owned()).map_err(|e| e.to_string())?;
    Ok(result)
}

pub fn lli_f64(program_str: &str) -> Result<f64, String> {
    let value = call_lli(program_str)?;
    print!("{}", value);
    match value.parse::<f64>() {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    }
}

pub fn lli_i1(program_str: &str) -> Result<bool, String> {
    let value = call_lli(program_str)?;
    print!("{}", value);
    match value.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("Unexpected boolean output: {}", value)),
    }
}
