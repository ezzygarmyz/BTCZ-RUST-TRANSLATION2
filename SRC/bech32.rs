use bech32::{self, FromBase32, ToBase32, Variant};

/// Encodes binary data into a Bech32 string with the given human-readable part (HRP).
pub fn encode_bech32(hrp: &str, data: &[u8]) -> Option<String> {
    bech32::encode(hrp, data.to_base32(), Variant::Bech32).ok()
}

/// Decodes a Bech32 string into its human-readable part (HRP) and binary data.
/// Returns `None` if the input string is invalid.
pub fn decode_bech32(encoded: &str) -> Option<(String, Vec<u8>)> {
    match bech32::decode(encoded) {
        Ok((hrp, data, Variant::Bech32)) => {
            let decoded_data = Vec::from_base32(&data).ok()?;
            Some((hrp, decoded_data))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_bech32() {
        let hrp = "bc"; // Human-readable part for Bitcoin addresses
        let data = vec![0, 1, 2, 3, 4, 5];
        let encoded = encode_bech32(hrp, &data).unwrap();
        assert_eq!(encoded, "bc1qypqypqypqypqypq");
    }

    #[test]
    fn test_decode_bech32() {
        let encoded = "bc1qypqypqypqypqypq";
        let (hrp, decoded_data) = decode_bech32(encoded).unwrap();
        assert_eq!(hrp, "bc");
        assert_eq!(decoded_data, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_invalid_bech32() {
        let invalid_encoded = "bc1qypqypqypqypq!"; // Invalid character
        assert!(decode_bech32(invalid_encoded).is_none());
    }
}
