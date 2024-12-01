use std::fs;
use std::path::Path;

/// Validates if the given file exists
pub fn validate_file_exists(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    if path.exists() && path.is_file() {
        Ok(())
    } else {
        Err(format!("File not found or is not a valid file: {}", file_path))
    }
}

/// Reads a file into a byte buffer
pub fn read_file_to_buffer(file_path: &str) -> Result<Vec<u8>, String> {
    fs::read(file_path).map_err(|e| format!("Failed to read file: {}", e))
}
