use std::mem::transmute;

#[inline]
pub fn is_little_endian() -> bool {
    cfg!(target_endian = "little")
}

#[inline]
pub fn is_big_endian() -> bool {
    cfg!(target_endian = "big")
}

/// Converts a 16-bit integer to little-endian.
pub fn to_little_endian_16(x: u16) -> u16 {
    if is_little_endian() {
        x
    } else {
        x.swap_bytes()
    }
}

/// Converts a 16-bit integer to big-endian.
pub fn to_big_endian_16(x: u16) -> u16 {
    if is_big_endian() {
        x
    } else {
        x.swap_bytes()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endian_detection() {
        if is_little_endian() {
            assert!(is_little_endian());
            assert!(!is_big_endian());
        } else {
            assert!(!is_little_endian());
            assert!(is_big_endian());
        }
    }

    #[test]
    fn test_endian_conversion() {
        assert_eq!(to_little_endian_16(0x1234), 0x3412);
    }
}
