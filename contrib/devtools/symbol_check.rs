use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;
use std::io;

/// Represents an error during symbol checking
#[derive(Debug)]
enum SymbolCheckError {
    Io(io::Error),
    CommandError(String),
}

/// Executes a command and captures its output
fn execute_command(command: &str, args: &[&str]) -> Result<String, SymbolCheckError> {
    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .map_err(SymbolCheckError::Io)?;

    if !output.status.success() {
        return Err(SymbolCheckError::CommandError(format!(
            "Command '{}' failed with args {:?}",
            command, args
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Checks symbols in a given binary file
fn check_symbols(binary_path: &str) -> Result<(), SymbolCheckError> {
    println!("Checking symbols in {}", binary_path);

    // Execute the `nm` command to list symbols
    let output = execute_command("nm", &[binary_path])?;
    let lines: Vec<&str> = output.lines().collect();

    let mut has_nonportable_symbols = false;

    for line in lines {
        if line.contains(" U ") {
            // Example: Check for undefined symbols
            println!("Non-portable symbol found: {}", line);
            has_nonportable_symbols = true;
        }
    }

    if has_nonportable_symbols {
        println!("Non-portable symbols detected in {}", binary_path);
    } else {
        println!("No issues found in {}", binary_path);
    }

    Ok(())
}

fn main() -> Result<(), SymbolCheckError> {
    // Specify the directory containing binaries
    let binary_dir = "target/release";
    if !Path::new(binary_dir).exists() {
        eprintln!("Binary directory '{}' does not exist.", binary_dir);
        return Ok(());
    }

    // Iterate over all binaries in the directory
    let entries = fs::read_dir(binary_dir).map_err(SymbolCheckError::Io)?;

    for entry in entries {
        let entry = entry.map_err(SymbolCheckError::Io)?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "exe" || ext == "" {
                    // Check symbols for each binary
                    check_symbols(path.to_str().unwrap())?;
                }
            }
        }
    }

    Ok(())
}
