use std::collections::HashMap;
use crate::transactions::Transaction;
use crate::keys::KeyPair;

pub struct Utxo {
    pub txid: String,
    pub amount: u64,
}

pub struct UtxoSet {
    utxos: HashMap<String, Utxo>,
}

impl UtxoSet {
    pub fn new() -> Self {
        UtxoSet {
            utxos: HashMap::new(),
        }
    }

    pub fn add(&mut self, txid: String, amount: u64) {
        self.utxos.insert(txid.clone(), Utxo { txid, amount });
    }

    pub fn calculate_balance(&self) -> u64 {
        self.utxos.values().map(|utxo| utxo.amount).sum()
    }

    pub fn create_transaction(
        &self,
        key_pair: &KeyPair,
        to_address: &str,
        amount: u64,
        fee: u64,
    ) -> Option<Transaction> {
        let mut total_input = 0;
        let mut inputs = vec![];

        for (txid, utxo) in &self.utxos {
            total_input += utxo.amount;
            inputs.push(txid.clone());
            if total_input >= amount + fee {
                break;
            }
        }

        if total_input < amount + fee {
            return None; // Insufficient funds
        }

        Some(Transaction::new(
            inputs,
            to_address.to_string(),
            amount,
            fee,
            key_pair,
        ))
    }
}
