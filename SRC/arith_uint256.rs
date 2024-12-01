use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul, Sub, Shl, Shr};

/// Represents a 256-bit unsigned integer for arithmetic operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArithUint256(BigUint);

impl ArithUint256 {
    /// Creates a new `ArithUint256` from a `BigUint`.
    pub fn new(value: BigUint) -> Self {
        ArithUint256(value)
    }

    /// Creates an `ArithUint256` initialized to zero.
    pub fn zero() -> Self {
        ArithUint256(BigUint::zero())
    }

    /// Creates an `ArithUint256` initialized to one.
    pub fn one() -> Self {
        ArithUint256(BigUint::one())
    }

    /// Converts the number to its compact representation.
    pub fn to_compact(&self) -> u32 {
        let mut size = (self.0.bits() + 7) / 8;
        let mut compact: u32 = 0;

        if size > 3 {
            let bytes = self.0.to_bytes_be();
            compact = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
            compact |= size << 24;
        } else {
            compact = self.0.to_u32_digits().1[0];
            compact |= size << 24;
        }

        compact
    }

    /// Converts from a compact representation.
    pub fn from_compact(compact: u32) -> Self {
        let size = (compact >> 24) & 0xff;
        let mut value = compact & 0x007fffff;

        if size > 3 {
            value <<= 8 * (size - 3);
        }

        ArithUint256(BigUint::from(value))
    }
}

/// Implement arithmetic operators for `ArithUint256`.
impl Add for ArithUint256 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ArithUint256(self.0 + other.0)
    }
}

impl Sub for ArithUint256 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ArithUint256(self.0 - other.0)
    }
}

impl Mul for ArithUint256 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ArithUint256(self.0 * other.0)
    }
}

impl Div for ArithUint256 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        ArithUint256(self.0 / other.0)
    }
}

impl Shl<u32> for ArithUint256 {
    type Output = Self;

    fn shl(self, shift: u32) -> Self {
        ArithUint256(self.0 << shift)
    }
}

impl Shr<u32> for ArithUint256 {
    type Output = Self;

    fn shr(self, shift: u32) -> Self {
        ArithUint256(self.0 >> shift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_arith_uint256_addition() {
        let a = ArithUint256::new(BigUint::from(10u32));
        let b = ArithUint256::new(BigUint::from(20u32));
        let result = a + b;
        assert_eq!(result.0, BigUint::from(30u32));
    }

    #[test]
    fn test_arith_uint256_compact_conversion() {
        let number = ArithUint256::new(BigUint::from(0x1d00ffffu32));
        let compact = number.to_compact();
        let restored = ArithUint256::from_compact(compact);
        assert_eq!(number, restored);
    }
}
