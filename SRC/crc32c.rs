use crc32fast::Hasher;

/// Computes CRC32C for the given input data.
pub fn compute_crc32c(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

/// Verifies CRC32C checksum.
pub fn verify_crc32c(data: &[u8], checksum: u32) -> bool {
    compute_crc32c(data) == checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32c() {
        let data = b"BitcoinZ CRC32C test";
        let checksum = compute_crc32c(data);
        assert!(verify_crc32c(data, checksum));
    }
}
