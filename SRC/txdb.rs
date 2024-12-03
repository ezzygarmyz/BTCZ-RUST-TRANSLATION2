use crate::utxo::{OutPoint, Utxo};
use std::collections::HashMap;
use thiserror::Error;

/// Custom errors for the transaction database
#[derive(Debug, Error)]
pub enum TxDbError {
    #[error("Transaction not found")]
    TxNotFound,
    #[error("Database operation failed")]
    DbError,
}

/// Persistent transaction database
pub struct TxDb {
    utxo_set: HashMap<OutPoint, Utxo>,
}

impl TxDb {
    /// Creates a new empty TxDb
    pub fn new() -> Self {
        TxDb {
            utxo_set: HashMap::new(),
        }
    }

    /// Adds a UTXO to the database
    pub fn add_utxo(&mut self, utxo: Utxo) {
        self.utxo_set.insert(utxo.outpoint.clone(), utxo);
    }

    /// Removes a UTXO from the database (marks it as spent)
    pub fn remove_utxo(&mut self, outpoint: &OutPoint) -> Result<(), TxDbError> {
        if self.utxo_set.remove(outpoint).is_none() {
            return Err(TxDbError::TxNotFound);
        }
        Ok(())
    }

    /// Retrieves a UTXO from the database
    pub fn get_utxo(&self, outpoint: &OutPoint) -> Option<&Utxo> {
        self.utxo_set.get(outpoint)
    }

    /// Validates and updates the UTXO set for a block
    pub fn update_utxo_set(&mut self, block_utxos: Vec<Utxo>, spent_outpoints: Vec<OutPoint>) -> Result<(), TxDbError> {
        // Add new UTXOs
        for utxo in block_utxos {
            self.add_utxo(utxo);
        }

        // Remove spent outputs
        for outpoint in spent_outpoints {
            self.remove_utxo(&outpoint)?;
        }

        Ok(())
    }
}
