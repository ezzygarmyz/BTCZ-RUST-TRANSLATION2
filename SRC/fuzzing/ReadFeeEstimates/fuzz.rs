use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use crate::fee_estimates::process_fee_estimates;

/// Fuzzing logic to read, process, and test fee estimate data
pub fn run_fuzzing(file_path: &str) -> Result<(), String> {
    let file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut buffer = Vec::new();
    
    // Read the file content
    let mut reader = io::BufReader::new(file);
    reader.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file: {}", e))?;

    // Process the fuzzing input data
    process_fee_estimates(&buffer)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: fuzz <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    if let Err(e) = run_fuzzing(file_path) {
        eprintln!("Error during fuzzing: {}", e);
        std::process::exit(1);
    }

    println!("Fuzzing completed successfully.");
}
