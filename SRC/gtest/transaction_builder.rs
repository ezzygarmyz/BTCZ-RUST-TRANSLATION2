#[cfg(test)]
mod tests {
    use crate::transaction_builder::TransactionBuilder;

    #[test]
    fn create_transaction() {
        let builder = TransactionBuilder::new();
        let tx = builder.build();
        assert!(tx.is_valid());
    }
}

pub mod transaction_builder {
    use crate::transaction::Transaction;

    pub struct TransactionBuilder;

    impl TransactionBuilder {
        pub fn new() -> Self {
            TransactionBuilder
        }

        pub fn build(&self) -> Transaction {
            Transaction {
                vin: vec![],
                vout: vec![],
            }
        }
    }

    pub mod transaction {
        #[derive(Default)]
        pub struct Transaction {
            pub vin: Vec<String>,
            pub vout: Vec<String>,
        }

        impl Transaction {
            pub fn is_valid(&self) -> bool {
                !self.vin.is_empty() && !self.vout.is_empty()
            }
        }
    }
}
