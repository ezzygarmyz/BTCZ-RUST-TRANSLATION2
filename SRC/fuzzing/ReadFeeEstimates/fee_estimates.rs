use std::collections::HashMap;
use std::str;

#[derive(Debug)]
pub struct FeeEstimate {
    pub block: u32,
    pub fee: f64,
}

/// Processes and validates fee estimate data from a byte buffer
pub fn process_fee_estimates(buffer: &[u8]) -> Result<(), String> {
    let data_str = str::from_utf8(buffer).map_err(|e| format!("Invalid UTF-8 data: {}", e))?;

    // Parse lines
    let mut fee_map: HashMap<u32, FeeEstimate> = HashMap::new();
    for line in data_str.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Malformed line: {}", line));
        }

        let block: u32 = parts[0]
            .trim()
            .parse()
            .map_err(|e| format!("Invalid block number: {}", e))?;
        let fee: f64 = parts[1]
            .trim()
            .parse()
            .map_err(|e| format!("Invalid fee value: {}", e))?;

        let estimate = FeeEstimate { block, fee };
        fee_map.insert(block, estimate);
    }

    validate_fee_estimates(&fee_map)?;

    Ok(())
}

/// Validates fee estimates for correctness
fn validate_fee_estimates(fee_map: &HashMap<u32, FeeEstimate>) -> Result<(), String> {
    for (block, estimate) in fee_map.iter() {
        if estimate.fee <= 0.0 {
            return Err(format!("Invalid fee at block {}: Fee must be positive", block));
        }
    }
    println!("All fee estimates are valid.");
    Ok(())
}
