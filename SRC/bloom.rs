use bloom::{BloomFilter, ASMSHasher};
use serde::{Deserialize, Serialize};

/// Represents a Bloom filter for efficient membership testing.
#[derive(Debug, Serialize, Deserialize)]
pub struct BitcoinBloomFilter {
    filter: BloomFilter,
}

impl BitcoinBloomFilter {
    /// Creates a new Bloom filter with the given size and false-positive rate.
    pub fn new(size: usize, false_positive_rate: f64) -> Self {
        let filter = BloomFilter::with_rate(false_positive_rate, size);
        BitcoinBloomFilter { filter }
    }

    /// Adds data to the Bloom filter.
    pub fn insert(&mut self, data: &[u8]) {
        self.filter.insert(data);
    }

    /// Checks if the given data is in the Bloom filter.
    pub fn contains(&self, data: &[u8]) -> bool {
        self.filter.contains(data)
    }

    /// Serializes the Bloom filter into a byte vector.
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).expect("Serialization failed")
    }

    /// Deserializes a Bloom filter from a byte vector.
    pub fn deserialize(data: &[u8]) -> Self {
        bincode::deserialize(data).expect("Deserialization failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter() {
        let mut bloom = BitcoinBloomFilter::new(100, 0.01);

        let item1 = b"transaction1";
        let item2 = b"transaction2";

        bloom.insert(item1);

        assert!(bloom.contains(item1));
        assert!(!bloom.contains(item2));
    }

    #[test]
    fn test_serialization() {
        let mut bloom = BitcoinBloomFilter::new(100, 0.01);

        let item = b"transaction1";
        bloom.insert(item);

        let serialized = bloom.serialize();
        let deserialized = BitcoinBloomFilter::deserialize(&serialized);

        assert!(deserialized.contains(item));
    }
}
