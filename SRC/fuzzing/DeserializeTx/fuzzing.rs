mod deserialize;
mod transaction;

use deserialize::deserialize_transaction;
use transaction::Transaction;

/// Fuzz test function for transaction deserialization.
pub fn fuzz_transaction(data: &[u8]) -> Result<Transaction, String> {
    // Attempt to deserialize the transaction from the input data
    match deserialize_transaction(data) {
        Ok(tx) => {
            println!("Deserialized transaction: {:?}", tx);
            Ok(tx)
        }
        Err(e) => {
            println!("Deserialization failed: {}", e);
            Err(e)
        }
    }
}
