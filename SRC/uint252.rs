use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, Shl, Shr};

/// A 252-bit unsigned integer
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uint252 {
    high: u128, // Upper 128 bits
    low: u128,  // Lower 124 bits (252 bits total)
}

impl Uint252 {
    /// Creates a new Uint252 from high and low parts
    pub fn new(high: u128, low: u128) -> Self {
        Uint252 { high, low: low & ((1 << 124) - 1) }
    }

    /// Returns a Uint252 from a single u64 value
    pub fn from_u64(value: u64) -> Self {
        Uint252 { high: 0, low: value as u128 }
    }

    /// Returns the high 128 bits
    pub fn high(&self) -> u128 {
        self.high
    }

    /// Returns the low 124 bits
    pub fn low(&self) -> u128 {
        self.low
    }
}

/// Display implementation for Uint252
impl fmt::Display for Uint252 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:032x}{:016x}", self.high, self.low)
    }
}

/// Basic arithmetic operations
impl Add for Uint252 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (low, carry) = self.low.overflowing_add(other.low);
        let high = self.high + other.high + carry as u128;
        Uint252::new(high, low)
    }
}

impl Sub for Uint252 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (low, borrow) = self.low.overflowing_sub(other.low);
        let high = self.high - other.high - borrow as u128;
        Uint252::new(high, low)
    }
}

impl Mul for Uint252 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let low = self.low.wrapping_mul(other.low);
        let high = self.high.wrapping_mul(other.high)
            + (self.high.wrapping_mul(other.low) >> 124)
            + (self.low.wrapping_mul(other.high) >> 124);
        Uint252::new(high, low)
    }
}

impl Div for Uint252 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // Simplistic approach for demonstration purposes
        if other.low == 0 && other.high == 0 {
            panic!("Division by zero");
        }

        let combined_self = ((self.high as u256) << 124) | self.low as u256;
        let combined_other = ((other.high as u256) << 124) | other.low as u256;

        let result = combined_self / combined_other;
        Uint252::new((result >> 124) as u128, result as u128)
    }
}
