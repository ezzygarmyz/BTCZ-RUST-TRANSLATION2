use std::num::ParseFloatError;

/// Number of satoshis in one BTC
const SATOSHIS_PER_BTC: u64 = 100_000_000;

/// Converts a satoshi value to a human-readable string
pub fn format_money(amount: u64) -> String {
    let btc = amount as f64 / SATOSHIS_PER_BTC as f64;
    format!("{:.8} BTC", btc)
}

/// Parses a human-readable money string into satoshis
pub fn parse_money(input: &str) -> Result<u64, ParseFloatError> {
    let trimmed_input = input.trim();
    let btc_value: f64 = trimmed_input.parse()?;
    Ok((btc_value * SATOSHIS_PER_BTC as f64).round() as u64)
}
