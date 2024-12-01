#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;

    #[test]
    fn default_transaction() {
        let tx = Transaction::default();
        assert_eq!(tx.vin.len(), 0);
        assert_eq!(tx.vout.len(), 0);
    }
}

pub mod transaction {
    #[derive(Default)]
    pub struct Transaction {
        pub vin: Vec<String>,  // Replace `String` with actual input structure
        pub vout: Vec<String>, // Replace `String` with actual output structure
    }
}
