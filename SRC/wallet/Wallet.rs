use std::collections::HashMap;
use crate::keys::KeyPair;
use crate::utxo::UtxoSet;
use crate::transactions::Transaction;

pub struct Wallet {
    pub address: String,
    pub balance: u64,
    pub utxos: UtxoSet,
    pub key_pair: KeyPair,
}

impl Wallet {
    /// Create a new wallet with a random key pair
    pub fn new() -> Self {
        let key_pair = KeyPair::generate();
        let address = key_pair.get_address();
        Wallet {
            address,
            balance: 0,
            utxos: UtxoSet::new(),
            key_pair,
        }
    }

    /// Get the wallet's balance by summing the UTXOs
    pub fn get_balance(&self) -> u64 {
        self.utxos.calculate_balance()
    }

    /// Add a UTXO to the wallet
    pub fn add_utxo(&mut self, txid: String, amount: u64) {
        self.utxos.add(txid, amount);
        self.balance = self.get_balance();
    }

    /// Send funds to another address
    pub fn create_transaction(&self, to_address: &str, amount: u64, fee: u64) -> Option<Transaction> {
        if amount + fee > self.balance {
            return None; // Insufficient funds
        }
        self.utxos.create_transaction(&self.key_pair, to_address, amount, fee)
    }
}
