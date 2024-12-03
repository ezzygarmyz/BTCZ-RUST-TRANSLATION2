use std::collections::BTreeMap;
use crate::blockchain::Block;

/// TimestampIndex for managing block lookups by timestamp
pub struct TimestampIndex {
    index: BTreeMap<u64, Vec<String>>, // Maps timestamps to block hashes
}

impl TimestampIndex {
    /// Creates a new empty TimestampIndex
    pub fn new() -> Self {
        TimestampIndex {
            index: BTreeMap::new(),
        }
    }

    /// Adds a block's timestamp and hash to the index
    pub fn add_block(&mut self, timestamp: u64, block_hash: String) {
        self.index
            .entry(timestamp)
            .or_insert_with(Vec::new)
            .push(block_hash);
    }

    /// Retrieves block hashes for a given timestamp
    pub fn get_blocks_by_timestamp(&self, timestamp: u64) -> Option<&Vec<String>> {
        self.index.get(&timestamp)
    }

    /// Retrieves all block hashes within a timestamp range (inclusive)
    pub fn get_blocks_in_range(&self, start: u64, end: u64) -> Vec<String> {
        self.index
            .range(start..=end)
            .flat_map(|(_, hashes)| hashes.clone())
            .collect()
    }

    /// Returns the earliest timestamp in the index
    pub fn earliest_timestamp(&self) -> Option<u64> {
        self.index.keys().next().cloned()
    }

    /// Returns the latest timestamp in the index
    pub fn latest_timestamp(&self) -> Option<u64> {
        self.index.keys().next_back().cloned()
    }
}
