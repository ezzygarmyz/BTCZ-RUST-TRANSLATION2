use std::str::FromStr;
use thiserror::Error;
use crate::base58::{Base58, Base58Error};
use crate::keys::{PublicKey, PrivateKey};
use crate::script::Script;

// Custom errors for KeyIO operations
#[derive(Debug, Error)]
pub enum KeyIoError {
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Base58 decoding error: {0}")]
    Base58Error(#[from] Base58Error),
    #[error("Unsupported address type")]
    UnsupportedAddressType,
}

/// Represents different types of BitcoinZ addresses
#[derive(Debug, PartialEq, Eq)]
pub enum AddressType {
    P2PKH,
    P2SH,
}

/// Represents a BitcoinZ address
#[derive(Debug, PartialEq, Eq)]
pub struct Address {
    pub address_type: AddressType,
    pub hash: Vec<u8>,
}

impl Address {
    /// Parses a BitcoinZ address from a string
    pub fn from_str(address: &str) -> Result<Self, KeyIoError> {
        let decoded = Base58::decode(address)?;
        if decoded.len() < 5 {
            return Err(KeyIoError::InvalidAddress);
        }

        // Validate checksum
        let (payload, checksum) = decoded.split_at(decoded.len() - 4);
        let calculated_checksum = double_sha256(payload)[..4].to_vec();
        if checksum != calculated_checksum {
            return Err(KeyIoError::InvalidAddress);
        }

        // Determine address type
        match payload[0] {
            0x1C => Ok(Self {
                address_type: AddressType::P2PKH,
                hash: payload[1..].to_vec(),
            }),
            0x1D => Ok(Self {
                address_type: AddressType::P2SH,
                hash: payload[1..].to_vec(),
            }),
            _ => Err(KeyIoError::UnsupportedAddressType),
        }
    }

    /// Encodes an address to a string
    pub fn to_string(&self) -> String {
        let mut payload = vec![];
        payload.push(match self.address_type {
            AddressType::P2PKH => 0x1C,
            AddressType::P2SH => 0x1D,
        });
        payload.extend(&self.hash);

        let checksum = double_sha256(&payload)[..4].to_vec();
        payload.extend(&checksum);

        Base58::encode(&payload)
    }
}

/// Utility function to perform a double SHA256 hash
fn double_sha256(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let hash_once = Sha256::digest(data);
    let hash_twice = Sha256::digest(&hash_once);
    hash_twice.to_vec()
}
