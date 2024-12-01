use sha2::{Digest, Sha256};
use ripemd160::Ripemd160;

/// Performs a single SHA-256 hash on the input data.
pub fn sha256(data: &[u8]) -> Vec<u8> {
    Sha256::digest(data).to_vec()
}

/// Performs a double SHA-256 hash on the input data.
pub fn double_sha256(data: &[u8]) -> Vec<u8> {
    let first_hash = Sha256::digest(data);
    Sha256::digest(&first_hash).to_vec()
}

/// Performs a RIPEMD-160 hash on the input data.
pub fn ripemd160(data: &[u8]) -> Vec<u8> {
    Ripemd160::digest(data).to_vec()
}

/// Computes a SHA-256 hash followed by a RIPEMD-160 hash.
pub fn hash160(data: &[u8]) -> Vec<u8> {
    let sha256_hash = Sha256::digest(data);
    Ripemd160::digest(&sha256_hash).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let data = b"hello world";
        let hash = sha256(data);
        assert_eq!(
            hex::encode(hash),
            "b94d27b9934d3e08a52e52d7da7dabfade83d41e2f3a6bda5fb6e3a0eb3f9a95"
        );
    }

    #[test]
    fn test_double_sha256() {
        let data = b"hello world";
        let hash = double_sha256(data);
        assert_eq!(
            hex::encode(hash),
            "9b64f11d2e11db6d5dfed1ea3c4a1db75f1c5d5636de534f79593ff0a04d37b8"
        );
    }

    #[test]
    fn test_ripemd160() {
        let data = b"hello world";
        let hash = ripemd160(data);
        assert_eq!(
            hex::encode(hash),
            "98c615784ccb5fe5936fbc0cbe9dfdb408d92f0f"
        );
    }

    #[test]
    fn test_hash160() {
        let data = b"hello world";
        let hash = hash160(data);
        assert_eq!(
            hex::encode(hash),
            "b6a9c8c230722b7c748331a8b450f05566dc7d0f"
        );
    }
}
