use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

/// Represents a transaction in the mempool
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub txid: String,
    pub fee: u64,   // Fee in satoshis
    pub size: usize, // Size in bytes
}

/// Mempool with memory usage limits
pub struct Mempool {
    transactions: Arc<Mutex<HashMap<String, Transaction>>>,
    fee_index: Arc<Mutex<BTreeMap<u64, Vec<String>>>>, // Fee -> List of txids
    total_memory: Arc<Mutex<usize>>,
    max_memory: usize,
}

impl Mempool {
    /// Creates a new mempool with a specified memory limit
    pub fn new(max_memory: usize) -> Self {
        Mempool {
            transactions: Arc::new(Mutex::new(HashMap::new())),
            fee_index: Arc::new(Mutex::new(BTreeMap::new())),
            total_memory: Arc::new(Mutex::new(0)),
            max_memory,
        }
    }

    /// Adds a transaction to the mempool
    pub fn add_transaction(&self, tx: Transaction) {
        let mut transactions = self.transactions.lock().unwrap();
        let mut fee_index = self.fee_index.lock().unwrap();
        let mut total_memory = self.total_memory.lock().unwrap();

        // Check if transaction already exists
        if transactions.contains_key(&tx.txid) {
            return;
        }

        // Add transaction
        *total_memory += tx.size;
        transactions.insert(tx.txid.clone(), tx.clone());

        // Update fee index
        fee_index.entry(tx.fee).or_insert_with(Vec::new).push(tx.txid.clone());

        // Prune if necessary
        self.prune_mempool(&mut transactions, &mut fee_index, &mut total_memory);
    }

    /// Prunes the mempool to maintain memory limits
    fn prune_mempool(
        &self,
        transactions: &mut HashMap<String, Transaction>,
        fee_index: &mut BTreeMap<u64, Vec<String>>,
        total_memory: &mut usize,
    ) {
        while *total_memory > self.max_memory {
            if let Some((&lowest_fee, txids)) = fee_index.iter_mut().next() {
                if let Some(txid) = txids.pop() {
                    if txids.is_empty() {
                        fee_index.remove(&lowest_fee);
                    }

                    if let Some(tx) = transactions.remove(&txid) {
                        *total_memory -= tx.size;
                    }
                }
            } else {
                break;
            }
        }
    }

    /// Retrieves a transaction by its ID
    pub fn get_transaction(&self, txid: &str) -> Option<Transaction> {
        let transactions = self.transactions.lock().unwrap();
        transactions.get(txid).cloned()
    }

    /// Returns the current memory usage
    pub fn current_memory_usage(&self) -> usize {
        let total_memory = self.total_memory.lock().unwrap();
        *total_memory
    }

    /// Returns the maximum memory limit
    pub fn max_memory_limit(&self) -> usize {
        self.max_memory
    }
}
