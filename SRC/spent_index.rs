use std::collections::HashMap;
use crate::utils::hash::Hash256;
use crate::transaction::OutPoint;

/// Represents a spent transaction output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpentOutput {
    pub txid: Hash256,       // Transaction ID of the spending transaction
    pub index: u32,          // Output index in the spending transaction
    pub height: u32,         // Block height where the spending occurred
    pub is_coinbase: bool,   // Indicates if the spending transaction is a coinbase
}

impl SpentOutput {
    /// Creates a new SpentOutput
    pub fn new(txid: Hash256, index: u32, height: u32, is_coinbase: bool) -> Self {
        SpentOutput {
            txid,
            index,
            height,
            is_coinbase,
        }
    }
}

/// Spent Transaction Output Index for tracking spent outputs
pub struct SpentIndex {
    spent_outputs: HashMap<OutPoint, SpentOutput>,
}

impl SpentIndex {
    /// Creates a new empty SpentIndex
    pub fn new() -> Self {
        SpentIndex {
            spent_outputs: HashMap::new(),
        }
    }

    /// Adds a spent output to the index
    pub fn add_spent_output(&mut self, outpoint: OutPoint, spent_output: SpentOutput) {
        self.spent_outputs.insert(outpoint, spent_output);
    }

    /// Retrieves a spent output by its OutPoint
    pub fn get_spent_output(&self, outpoint: &OutPoint) -> Option<&SpentOutput> {
        self.spent_outputs.get(outpoint)
    }

    /// Checks if an OutPoint is spent
    pub fn is_spent(&self, outpoint: &OutPoint) -> bool {
        self.spent_outputs.contains_key(outpoint)
    }

    /// Removes a spent output from the index
    pub fn remove_spent_output(&mut self, outpoint: &OutPoint) -> bool {
        self.spent_outputs.remove(outpoint).is_some()
    }
}
