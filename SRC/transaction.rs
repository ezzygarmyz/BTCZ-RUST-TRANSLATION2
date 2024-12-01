use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut};
use bitcoin::consensus::encode::{deserialize, serialize};
use bitcoin::util::hash::Sha256dHash;
use bitcoin::Script;
use std::io::{self, Write};

/// Represents a BitcoinZ transaction.
pub struct BitcoinZTransaction {
    pub tx: Transaction,
}

impl BitcoinZTransaction {
    /// Creates a new transaction.
    pub fn new(inputs: Vec<TxIn>, outputs: Vec<TxOut>) -> Self {
        let tx = Transaction {
            version: 2,
            lock_time: 0,
            input: inputs,
            output: outputs,
        };
        BitcoinZTransaction { tx }
    }

    /// Serializes the transaction into binary format.
    pub fn serialize(&self) -> Vec<u8> {
        serialize(&self.tx).unwrap()
    }

    /// Deserializes a transaction from binary format.
    pub fn deserialize(data: &[u8]) -> Result<Self, io::Error> {
        let tx: Transaction = deserialize(data).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid transaction"))?;
        Ok(BitcoinZTransaction { tx })
    }

    /// Computes the transaction hash (TxID).
    pub fn get_hash(&self) -> Sha256dHash {
        self.tx.txid()
    }

    /// Validates the transaction.
    pub fn is_valid(&self) -> bool {
        !self.tx.input.is_empty() && !self.tx.output.is_empty() // Example validation logic
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::blockdata::transaction::TxIn;
    use bitcoin::blockdata::transaction::TxOut;
    use bitcoin::OutPoint;

    #[test]
    fn test_transaction_creation() {
        let inputs = vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: Script::new(),
            sequence: 0xFFFFFFFF,
            witness: vec![],
        }];

        let outputs = vec![TxOut {
            value: 5000,
            script_pubkey: Script::new(),
        }];

        let tx = BitcoinZTransaction::new(inputs.clone(), outputs.clone());
        assert_eq!(tx.tx.input, inputs);
        assert_eq!(tx.tx.output, outputs);
    }

    #[test]
    fn test_transaction_serialization() {
        let inputs = vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: Script::new(),
            sequence: 0xFFFFFFFF,
            witness: vec![],
        }];

        let outputs = vec![TxOut {
            value: 5000,
            script_pubkey: Script::new(),
        }];

        let tx = BitcoinZTransaction::new(inputs, outputs);

        let serialized = tx.serialize();
        let deserialized = BitcoinZTransaction::deserialize(&serialized).unwrap();

        assert_eq!(tx.tx, deserialized.tx);
    }

    #[test]
    fn test_transaction_hash() {
        let inputs = vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: Script::new(),
            sequence: 0xFFFFFFFF,
            witness: vec![],
        }];

        let outputs = vec![TxOut {
            value: 5000,
            script_pubkey: Script::new(),
        }];

        let tx = BitcoinZTransaction::new(inputs, outputs);
        let hash = tx.get_hash();

        assert!(!hash.to_string().is_empty());
    }

    #[test]
    fn test_transaction_validation() {
        let inputs = vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: Script::new(),
            sequence: 0xFFFFFFFF,
            witness: vec![],
        }];

        let outputs = vec![TxOut {
            value: 5000,
            script_pubkey: Script::new(),
        }];

        let tx = BitcoinZTransaction::new(inputs, outputs);
        assert!(tx.is_valid());

        let invalid_tx = BitcoinZTransaction::new(vec![], vec![]);
        assert!(!invalid_tx.is_valid());
    }
}
