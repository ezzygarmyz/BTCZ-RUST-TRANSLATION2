use crate::transaction::Transaction;
use std::io::{Cursor, Read};

/// Deserializes a transaction from a byte stream.
pub fn deserialize_transaction(data: &[u8]) -> Result<Transaction, String> {
    let mut cursor = Cursor::new(data);

    // Read and parse the transaction structure
    Transaction::from_stream(&mut cursor)
}
