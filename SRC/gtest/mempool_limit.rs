#[cfg(test)]
mod tests {
    use crate::mempool_limit::MemPool;

    #[test]
    fn enforce_limit() {
        let mut mempool = MemPool::new(10);
        for _ in 0..20 {
            mempool.add_transaction();
        }
        assert_eq!(mempool.size(), 10);
    }
}

pub mod mempool_limit {
    use std::collections::VecDeque;

    pub struct MemPool {
        transactions: VecDeque<String>,
        limit: usize,
    }

    impl MemPool {
        pub fn new(limit: usize) -> Self {
            MemPool {
                transactions: VecDeque::new(),
                limit,
            }
        }

        pub fn add_transaction(&mut self) {
            if self.transactions.len() >= self.limit {
                self.transactions.pop_front();
            }
            self.transactions.push_back("tx".to_string());
        }

        pub fn size(&self) -> usize {
            self.transactions.len()
        }
    }
}
