use crate::transaction::Transaction;
use crate::utxo::OutPoint;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use thiserror::Error;

/// Custom errors for the mempool
#[derive(Debug, Error)]
pub enum MempoolError {
    #[error("Transaction not found")]
    TxNotFound,
    #[error("Transaction is invalid")]
    InvalidTransaction,
}

/// Represents a mempool transaction with its fee rate for prioritization
#[derive(Debug, Clone)]
pub struct MempoolTransaction {
    pub transaction: Transaction,
    pub fee_rate: u64, // Satoshis per byte
}

impl PartialEq for MempoolTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.fee_rate == other.fee_rate
    }
}

impl Eq for MempoolTransaction {}

impl PartialOrd for MempoolTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.fee_rate.cmp(&other.fee_rate).reverse()) // Higher fee rate first
    }
}

impl Ord for MempoolTransaction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fee_rate.cmp(&other.fee_rate).reverse()
    }
}

/// Transaction Mempool
pub struct Mempool {
    transactions: HashMap<String, MempoolTransaction>, // TxID -> MempoolTransaction
    priority_queue: BinaryHeap<Reverse<MempoolTransaction>>, // Fee-based priority
}

impl Mempool {
    /// Creates a new empty Mempool
    pub fn new() -> Self {
        Mempool {
            transactions: HashMap::new(),
            priority_queue: BinaryHeap::new(),
        }
    }

    /// Adds a transaction to the mempool
    pub fn add_transaction(&mut self, transaction: Transaction, fee_rate: u64) -> Result<(), MempoolError> {
        let txid = transaction.txid();
        if self.transactions.contains_key(&txid) {
            return Err(MempoolError::InvalidTransaction);
        }

        let mempool_tx = MempoolTransaction {
            transaction: transaction.clone(),
            fee_rate,
        };

        self.transactions.insert(txid.clone(), mempool_tx.clone());
        self.priority_queue.push(Reverse(mempool_tx));
        Ok(())
    }

    /// Retrieves a transaction by its ID
    pub fn get_transaction(&self, txid: &str) -> Option<&Transaction> {
        self.transactions.get(txid).map(|mempool_tx| &mempool_tx.transaction)
    }

    /// Removes a transaction from the mempool
    pub fn remove_transaction(&mut self, txid: &str) -> Result<(), MempoolError> {
        if let Some(mempool_tx) = self.transactions.remove(txid) {
            self.priority_queue = self
                .priority_queue
                .drain()
                .filter(|entry| entry.0.transaction.txid() != txid)
                .collect();
            Ok(())
        } else {
            Err(MempoolError::TxNotFound)
        }
    }

    /// Retrieves transactions ordered by fee rate
    pub fn get_highest_fee_transactions(&self) -> Vec<Transaction> {
        self.priority_queue
            .iter()
            .map(|entry| entry.0.transaction.clone())
            .collect()
    }
}
