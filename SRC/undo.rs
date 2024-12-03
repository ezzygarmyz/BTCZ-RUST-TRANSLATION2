use crate::transaction::TxOutput;
use crate::utxo::OutPoint;
use std::collections::HashMap;

/// Represents undo data for a single transaction input
#[derive(Debug, Clone)]
pub struct TxInUndo {
    pub is_coinbase: bool,
    pub height: u32,
    pub amount: u64,
    pub script_pubkey: Vec<u8>,
}

impl TxInUndo {
    /// Creates a new TxInUndo
    pub fn new(is_coinbase: bool, height: u32, amount: u64, script_pubkey: Vec<u8>) -> Self {
        TxInUndo {
            is_coinbase,
            height,
            amount,
            script_pubkey,
        }
    }
}

/// Represents undo data for an entire transaction
#[derive(Debug, Clone)]
pub struct TxUndo {
    pub inputs: Vec<TxInUndo>, // Undo data for each transaction input
}

impl TxUndo {
    /// Creates a new TxUndo
    pub fn new() -> Self {
        TxUndo { inputs: Vec::new() }
    }

    /// Adds undo data for a single input
    pub fn add_input(&mut self, undo: TxInUndo) {
        self.inputs.push(undo);
    }
}

/// Represents undo data for a block
#[derive(Debug, Clone)]
pub struct BlockUndo {
    pub tx_undos: HashMap<OutPoint, TxUndo>, // Undo data for transactions in the block
}

impl BlockUndo {
    /// Creates a new BlockUndo
    pub fn new() -> Self {
        BlockUndo {
            tx_undos: HashMap::new(),
        }
    }

    /// Adds undo data for a transaction
    pub fn add_tx_undo(&mut self, outpoint: OutPoint, tx_undo: TxUndo) {
        self.tx_undos.insert(outpoint, tx_undo);
    }

    /// Retrieves undo data for a transaction
    pub fn get_tx_undo(&self, outpoint: &OutPoint) -> Option<&TxUndo> {
        self.tx_undos.get(outpoint)
    }
}
