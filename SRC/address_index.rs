use serde::{Deserialize, Serialize};
use crate::uint256::Uint256; // Assuming `uint256` is defined elsewhere in the project.

/// Represents a key for the address index.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AddressIndexKey {
    pub hash_bytes: Uint256, // Address hash
    pub addr_type: i32,      // Address type
    pub height: i32,         // Block height
    pub txid: Uint256,       // Transaction ID
}

/// Represents a key for unspent outputs in the address index.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AddressUnspentKey {
    pub hash_bytes: Uint256, // Address hash
    pub addr_type: i32,      // Address type
    pub txid: Uint256,       // Transaction ID
    pub index: i32,          // Output index
}

/// Represents a value for unspent outputs in the address index.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressUnspentValue {
    pub satoshis: u64,  // Amount of satoshis
    pub script: Vec<u8>, // Serialized script
    pub height: i32,    // Block height
}
