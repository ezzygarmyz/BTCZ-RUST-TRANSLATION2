/// Swaps the byte order of a 16-bit unsigned integer.
pub fn bswap_16(x: u16) -> u16 {
    x.swap_bytes()
}

/// Swaps the byte order of a 32-bit unsigned integer.
pub fn bswap_32(x: u32) -> u32 {
    x.swap_bytes()
}

/// Swaps the byte order of a 64-bit unsigned integer.
pub fn bswap_64(x: u64) -> u64 {
    x.swap_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bswap_16() {
        assert_eq!(bswap_16(0x1234), 0x3412);
    }

    #[test]
    fn test_bswap_32() {
        assert_eq!(bswap_32(0x12345678), 0x78563412);
    }

    #[test]
    fn test_bswap_64() {
        assert_eq!(bswap_64(0x123456789ABCDEF0), 0xF0DEBC9A78563412);
    }
}
