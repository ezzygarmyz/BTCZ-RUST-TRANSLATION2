use std::fs::File;
use std::io::{self, Read};
use crate::validation::{check_block, ValidationError};
use crate::block::{Block, deserialize_block};

/// Fuzz tester for the CheckBlock functionality.
pub struct Fuzzer {
    input_data: Vec<u8>,
}

impl Fuzzer {
    /// Creates a new fuzzer with the provided input data.
    pub fn new(input_data: Vec<u8>) -> Self {
        Fuzzer { input_data }
    }

    /// Runs the fuzzing process by attempting to deserialize the block
    /// and validating it using the `CheckBlock` function.
    pub fn run(&self) -> Result<(), ValidationError> {
        // Attempt to deserialize the block from the input data
        let block = deserialize_block(&self.input_data)?;
        
        // Validate the block using the `CheckBlock` function
        check_block(&block)
    }
}

/// Reads input from a file for fuzzing.
pub fn read_input_from_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Entry point for the fuzzing process.
fn main() -> Result<(), ValidationError> {
    // Replace with an actual path to fuzzing input data
    let input_file_path = "fuzz_input.dat";
    
    // Load the fuzzing data
    let input_data = read_input_from_file(input_file_path)
        .expect("Failed to read fuzzing input data");
    
    // Create a fuzzer instance
    let fuzzer = Fuzzer::new(input_data);

    // Run the fuzzing process
    match fuzzer.run() {
        Ok(_) => println!("Fuzzing passed with no errors."),
        Err(e) => println!("Fuzzing failed: {:?}", e),
    }

    Ok(())
}
