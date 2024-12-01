use std::net::SocketAddr;

/// Represents network parameters for BitcoinZ.
#[derive(Debug, Clone)]
pub struct ChainParams {
    pub network_name: String,
    pub magic_bytes: [u8; 4],
    pub default_port: u16,
    pub address_prefix: u8,
    pub genesis_block: Block,
    pub seed_nodes: Vec<SocketAddr>,
}

/// Represents a block in the blockchain.
#[derive(Debug, Clone)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u32,
}

/// Returns the parameters for the mainnet.
pub fn mainnet_params() -> ChainParams {
    ChainParams {
        network_name: "mainnet".to_string(),
        magic_bytes: [0x24, 0xe9, 0x27, 0x64], // Example magic bytes
        default_port: 8233,
        address_prefix: 0x1c,
        genesis_block: create_genesis_block(
            "0000000000000000000", // Previous hash
            "4a5e1e",             // Merkle root
            1231006505,           // Timestamp
            0x1d00ffff,           // Bits
            2083236893,           // Nonce
        ),
        seed_nodes: vec![
            "127.0.0.1:8233".parse().unwrap(),
            "192.168.1.1:8233".parse().unwrap(),
        ],
    }
}

/// Returns the parameters for the testnet.
pub fn testnet_params() -> ChainParams {
    ChainParams {
        network_name: "testnet".to_string(),
        magic_bytes: [0x0b, 0x11, 0x09, 0x07],
        default_port: 18233,
        address_prefix: 0x6f,
        genesis_block: create_genesis_block(
            "0000000000000000000",
            "4a5e1e",
            1296688602,
            0x1d00ffff,
            414098458,
        ),
        seed_nodes: vec!["127.0.0.1:18233".parse().unwrap()],
    }
}

/// Creates a genesis block with the specified parameters.
fn create_genesis_block(
    previous_hash: &str,
    merkle_root: &str,
    timestamp: u64,
    bits: u32,
    nonce: u32,
) -> Block {
    Block {
        hash: calculate_genesis_hash(previous_hash, merkle_root, timestamp, bits, nonce),
        previous_hash: previous_hash.to_string(),
        merkle_root: merkle_root.to_string(),
        timestamp,
        bits,
        nonce,
    }
}

/// Calculates the genesis block hash (simplified example).
fn calculate_genesis_hash(
    previous_hash: &str,
    merkle_root: &str,
    timestamp: u64,
    bits: u32,
    nonce: u32,
) -> String {
    format!(
        "{}{}{}{}{}",
        previous_hash, merkle_root, timestamp, bits, nonce
    ) // Placeholder; replace with real hashing logic
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mainnet_params() {
        let params = mainnet_params();
        assert_eq!(params.network_name, "mainnet");
        assert_eq!(params.default_port, 8233);
        assert_eq!(params.magic_bytes, [0x24, 0xe9, 0x27, 0x64]);
    }

    #[test]
    fn test_genesis_block() {
        let block = create_genesis_block(
            "0000000000000000000",
            "4a5e1e",
            1231006505,
            0x1d00ffff,
            2083236893,
        );
        assert_eq!(
            block.hash,
            "00000000000000000004a5e112310065051d00ffff2083236893"
        ); // Example hash
    }
}
