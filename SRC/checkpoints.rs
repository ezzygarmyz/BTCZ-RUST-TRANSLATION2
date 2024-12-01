use std::collections::HashMap;

/// Represents a set of blockchain checkpoints.
pub struct Checkpoints {
    map_checkpoints: HashMap<u32, String>, // Block height to block hash
}

impl Checkpoints {
    /// Creates a new instance of `Checkpoints` with the given checkpoint data.
    pub fn new(checkpoints: Vec<(u32, &str)>) -> Self {
        let map_checkpoints = checkpoints
            .into_iter()
            .map(|(height, hash)| (height, hash.to_string()))
            .collect();
        Checkpoints { map_checkpoints }
    }

    /// Checks if the given block matches its checkpoint.
    pub fn check_block(&self, height: u32, hash: &str) -> bool {
        match self.map_checkpoints.get(&height) {
            Some(checkpoint_hash) => checkpoint_hash == hash,
            None => true, // No checkpoint for this height
        }
    }
}

/// Returns the mainnet checkpoints.
pub fn mainnet_checkpoints() -> Checkpoints {
    Checkpoints::new(vec![
        (0, "000000000019d6689c085ae165831e93"),
        (11111, "0000000069e244f73c25f66bf5b27c70"),
        // Add more mainnet checkpoints here
    ])
}

/// Returns the testnet checkpoints.
pub fn testnet_checkpoints() -> Checkpoints {
    Checkpoints::new(vec![
        (0, "000000000933ea01ad0ee984209779ba"),
        (546, "000000002a936ca763904c3c35fce2f2"),
        // Add more testnet checkpoints here
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_block() {
        let checkpoints = mainnet_checkpoints();

        assert!(checkpoints.check_block(0, "000000000019d6689c085ae165831e93"));
        assert!(!checkpoints.check_block(0, "00000000000000000000000000000000"));
        assert!(checkpoints.check_block(999999, "any_nonexistent_block_hash"));
    }

    #[test]
    fn test_testnet_checkpoints() {
        let checkpoints = testnet_checkpoints();

        assert!(checkpoints.check_block(0, "000000000933ea01ad0ee984209779ba"));
        assert!(checkpoints.check_block(999999, "any_nonexistent_block_hash"));
    }
}
