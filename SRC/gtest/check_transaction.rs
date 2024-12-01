#[cfg(test)]
mod tests {
    use crate::consensus::validation::check_transaction;
    use crate::primitives::transaction::Transaction;

    #[test]
    fn valid_transaction() {
        let tx = Transaction::default();
        assert!(check_transaction(&tx));
    }

    #[test]
    fn invalid_transaction() {
        let mut tx = Transaction::default();
        tx.vin.clear(); // Invalid: no inputs
        assert!(!check_transaction(&tx));
    }
}

pub mod consensus {
    pub mod validation {
        use crate::primitives::transaction::Transaction;

        pub fn check_transaction(tx: &Transaction) -> bool {
            !tx.vin.is_empty() && !tx.vout.is_empty()
        }
    }
}

pub mod primitives {
    #[derive(Default)]
    pub struct Transaction {
        pub vin: Vec<String>,  // Replace `String` with input struct
        pub vout: Vec<String>, // Replace `String` with output struct
    }
}
