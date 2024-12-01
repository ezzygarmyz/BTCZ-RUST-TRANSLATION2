use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;
use std::collections::HashMap;

/// Represents fee estimation statistics.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FeeEstimates {
    /// Mapping of confirmation targets (in blocks) to estimated fee rates (in satoshis per byte).
    pub estimates: HashMap<u32, f64>,
}

impl FeeEstimates {
    /// Reads fee estimates from the specified file.
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let estimates = serde_json::from_reader(reader)?;
        Ok(estimates)
    }

    /// Writes the current fee estimates to the specified file.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    /// Updates the fee estimate for a given confirmation target.
    pub fn update_estimate(&mut self, target: u32, fee_rate: f64) {
        self.estimates.insert(target, fee_rate);
    }

    /// Retrieves the fee estimate for a given confirmation target.
    pub fn get_estimate(&self, target: u32) -> Option<f64> {
        self.estimates.get(&target).cloned()
    }
}

fn main() -> io::Result<()> {
    // Example usage
    let path = "fee_estimates.dat";

    // Read existing estimates
    let mut fee_estimates = FeeEstimates::read_from_file(path).unwrap_or_default();

    // Update an estimate
    fee_estimates.update_estimate(6, 25.0); // 25 satoshis/byte for confirmation within 6 blocks

    // Write updated estimates back to file
    fee_estimates.write_to_file(path)?;

    // Retrieve an estimate
    if let Some(fee) = fee_estimates.get_estimate(6) {
        println!("Estimated fee for confirmation within 6 blocks: {} satoshis/byte", fee);
    } else {
        println!("No estimate available for the specified target.");
    }

    Ok(())
}
