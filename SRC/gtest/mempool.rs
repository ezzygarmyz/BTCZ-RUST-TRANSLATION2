#[cfg(test)]
mod tests {
    use crate::mempool::{MemPool, Transaction};

    #[test]
    fn add_transaction() {
        let tx = Transaction::default();
        let mut mempool = MemPool::new();
        mempool.add_transaction(tx);
        assert_eq!(mempool.size(), 1);
    }
}

pub mod mempool {
    use std::collections::HashMap;

    #[derive(Default, Clone)]
    pub struct Transaction {
        pub id: String,
    }

    pub struct MemPool {
        transactions: HashMap<String, Transaction>,
    }

    impl MemPool {
        pub fn new() -> Self {
            MemPool {
                transactions: HashMap::new(),
            }
        }

        pub fn add_transaction(&mut self, tx: Transaction) {
            self.transactions.insert(tx.id.clone(), tx);
        }

        pub fn size(&self) -> usize {
            self.transactions.len()
        }
    }
}
