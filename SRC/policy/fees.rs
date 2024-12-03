use crate::transaction::Transaction;

/// Minimum relay fee per kilobyte (in satoshis)
pub const MIN_RELAY_FEE_PER_KB: u64 = 1000; // 1000 satoshis per KB

/// Fee-related policy functions
pub struct FeePolicy;

impl FeePolicy {
    /// Calculates the required fee for a transaction based on its size
    pub fn calculate_fee(transaction: &Transaction) -> u64 {
        let size = transaction.get_size(); // Transaction size in bytes
        let fee = (size as u64 * MIN_RELAY_FEE_PER_KB + 999) / 1000; // Ceiling division
        fee
    }

    /// Validates that a transaction meets the minimum relay fee requirements
    pub fn validate_fee(transaction: &Transaction) -> bool {
        let required_fee = Self::calculate_fee(transaction);
        transaction.get_fee() >= required_fee
    }
}
