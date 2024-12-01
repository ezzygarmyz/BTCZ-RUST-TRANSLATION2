use base58::{FromBase58, ToBase58};

/// Encodes binary data into a Base58 string.
pub fn encode_base58(data: &[u8]) -> String {
    data.to_base58()
}

/// Decodes a Base58 string into binary data.
/// Returns `None` if the input string is not valid Base58.
pub fn decode_base58(encoded: &str) -> Option<Vec<u8>> {
    encoded.from_base58().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_base58() {
        let data = b"BitcoinZ";
        let encoded = encode_base58(data);
        assert_eq!(encoded, "BiEjcAU"); // Example output
    }

    #[test]
    fn test_decode_base58() {
        let encoded = "BiEjcAU";
        let decoded = decode_base58(encoded).unwrap();
        assert_eq!(decoded, b"BitcoinZ");
    }

    #[test]
    fn test_invalid_base58() {
        let invalid_encoded = "0OIl"; // Invalid Base58 characters
        assert!(decode_base58(invalid_encoded).is_none());
    }
}
use sha2::{Digest, Sha256};

/// Appends a 4-byte checksum to the data and encodes it in Base58.
pub fn encode_base58_with_checksum(data: &[u8]) -> String {
    let mut data_with_checksum = data.to_vec();
    let checksum = Sha256::digest(&Sha256::digest(data));
    data_with_checksum.extend_from_slice(&checksum[0..4]);
    encode_base58(&data_with_checksum)
}

/// Decodes a Base58 string with checksum validation.
/// Returns `None` if the checksum is invalid or input is not valid Base58.
pub fn decode_base58_with_checksum(encoded: &str) -> Option<Vec<u8>> {
    let data = decode_base58(encoded)?;
    if data.len() < 4 {
        return None;
    }

    let (payload, checksum) = data.split_at(data.len() - 4);
    let computed_checksum = &Sha256::digest(&Sha256::digest(payload))[0..4];
    if checksum == computed_checksum {
        Some(payload.to_vec())
    } else {
        None
    }
}

#[cfg(test)]
mod checksum_tests {
    use super::*;

    #[test]
    fn test_base58_with_checksum() {
        let data = b"BitcoinZ";
        let encoded = encode_base58_with_checksum(data);
        let decoded = decode_base58_with_checksum(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_invalid_checksum() {
        let encoded = "BiEjcAUinvalidchecksum";
        assert!(decode_base58_with_checksum(encoded).is_none());
    }
}
