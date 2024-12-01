/// Represents the smallest unit of BitcoinZ: Satoshis.
pub type Amount = i64;

/// Maximum allowed amount in the BitcoinZ network.
pub const MAX_MONEY: Amount = 21_000_000_000_000_000; // 21 million BTCZ in satoshis.

/// Checks if the given amount is within the valid monetary range.
pub fn is_valid_amount(value: Amount) -> bool {
    value >= 0 && value <= MAX_MONEY
}

/// Converts satoshis to BTCZ.
pub fn satoshis_to_btcz(satoshis: Amount) -> f64 {
    satoshis as f64 / 100_000_000.0
}

/// Converts BTCZ to satoshis.
pub fn btcz_to_satoshis(btcz: f64) -> Amount {
    (btcz * 100_000_000.0).round() as Amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_amount() {
        assert!(is_valid_amount(0));
        assert!(is_valid_amount(MAX_MONEY));
        assert!(!is_valid_amount(MAX_MONEY + 1));
        assert!(!is_valid_amount(-1));
    }

    #[test]
    fn test_conversion() {
        let satoshis = 100_000_000; // 1 BTCZ
        let btcz = 1.0;

        assert_eq!(satoshis_to_btcz(satoshis), btcz);
        assert_eq!(btcz_to_satoshis(btcz), satoshis);
    }
}
