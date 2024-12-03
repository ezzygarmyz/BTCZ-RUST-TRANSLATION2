use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, Shl, Shr};
use std::str::FromStr;

/// Represents a 256-bit unsigned integer
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uint256 {
    high: u128, // Upper 128 bits
    low: u128,  // Lower 128 bits
}

impl Uint256 {
    /// Creates a new Uint256 from high and low parts
    pub fn new(high: u128, low: u128) -> Self {
        Uint256 { high, low }
    }

    /// Converts a string (hexadecimal) to a Uint256
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let stripped_hex = hex.trim_start_matches("0x");
        if stripped_hex.len() > 64 {
            return Err("Hex string too large for Uint256");
        }

        let mut padded_hex = String::from("0").repeat(64 - stripped_hex.len());
        padded_hex.push_str(stripped_hex);

        let high = u128::from_str_radix(&padded_hex[0..32], 16).map_err(|_| "Invalid hex")?;
        let low = u128::from_str_radix(&padded_hex[32..64], 16).map_err(|_| "Invalid hex")?;

        Ok(Uint256 { high, low })
    }

    /// Converts a Uint256 to a hexadecimal string
    pub fn to_hex(&self) -> String {
        format!("{:032x}{:032x}", self.high, self.low)
    }

    /// Returns the high 128 bits
    pub fn high(&self) -> u128 {
        self.high
    }

    /// Returns the low 128 bits
    pub fn low(&self) -> u128 {
        self.low
    }
}

/// Display implementation for Uint256
impl fmt::Display for Uint256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:032x}{:032x}", self.high, self.low)
    }
}

/// Basic arithmetic operations
impl Add for Uint256 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (low, carry) = self.low.overflowing_add(other.low);
        let high = self.high + other.high + carry as u128;
        Uint256::new(high, low)
    }
}

impl Sub for Uint256 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (low, borrow) = self.low.overflowing_sub(other.low);
        let high = self.high - other.high - borrow as u128;
        Uint256::new(high, low)
    }
}

impl Mul for Uint256 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let low = self.low.wrapping_mul(other.low);
        let high = self.high.wrapping_mul(other.high)
            + (self.high.wrapping_mul(other.low) >> 128)
            + (self.low.wrapping_mul(other.high) >> 128);
        Uint256::new(high, low)
    }
}

impl Div for Uint256 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // Simplistic approach for demonstration purposes
        if other.low == 0 && other.high == 0 {
            panic!("Division by zero");
        }

        let combined_self = ((self.high as u256) << 128) | self.low as u256;
        let combined_other = ((other.high as u256) << 128) | other.low as u256;

        let result = combined_self / combined_other;
        Uint256::new((result >> 128) as u128, result as u128)
    }
}
