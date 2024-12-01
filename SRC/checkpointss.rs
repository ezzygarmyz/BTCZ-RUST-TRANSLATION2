use std::collections::HashMap;

/// Represents a checkpoint map where block height is mapped to its hash.
pub type Checkpoints = HashMap<u32, &'static str>;

/// Checkpoints for the mainnet.
pub const MAINNET_CHECKPOINTS: &[(&u32, &str)] = &[
    &(0, "000000000019d6689c085ae165831e93"),
    &(11111, "0000000069e244f73c25f66bf5b27c70"),
];

/// Checkpoints for the testnet.
pub const TESTNET_CHECKPOINTS: &[(&u32, &str)] = &[
    &(0, "000000000933ea01ad0ee984209779ba"),
    &(546, "000000002a936ca763904c3c35fce2f2"),
];

/// Converts an array of checkpoints into a HashMap.
pub fn build_checkpoint_map(checkpoints: &[(&u32, &str)]) -> Checkpoints {
    checkpoints.iter().cloned().collect()
}

/// Validates a block hash against the checkpoints.
pub fn check_block(height: u32, hash: &str, checkpoints: &Checkpoints) -> bool {
    match checkpoints.get(&height) {
        Some(&checkpoint_hash) => checkpoint_hash == hash,
        None => true, // No checkpoint for this height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_block() {
        let mainnet_checkpoints = build_checkpoint_map(MAINNET_CHECKPOINTS);
        let testnet_checkpoints = build_checkpoint_map(TESTNET_CHECKPOINTS);

        // Mainnet
        assert!(check_block(0, "000000000019d6689c085ae165831e93", &mainnet_checkpoints));
        assert!(!check_block(0, "00000000000000000000000000000000", &mainnet_checkpoints));

        // Testnet
        assert!(check_block(0, "000000000933ea01ad0ee984209779ba", &testnet_checkpoints));
        assert!(!check_block(546, "00000000000000000000000000000000", &testnet_checkpoints));

        // Non-existent checkpoint
        assert!(check_block(99999, "any_hash", &mainnet_checkpoints));
    }
}
