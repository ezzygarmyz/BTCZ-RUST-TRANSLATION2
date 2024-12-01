use crate::blockchain::{Block, BlockHeader};
use crate::mempool::Mempool;
use crate::consensus::{check_proof_of_work, calculate_next_work_required};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};

/// Represents a miner that prepares and mines blocks
pub struct Miner {
    mempool: Arc<Mempool>,
    difficulty_bits: u32,
}

impl Miner {
    /// Creates a new Miner instance
    pub fn new(mempool: Arc<Mempool>, difficulty_bits: u32) -> Self {
        Miner {
            mempool,
            difficulty_bits,
        }
    }

    /// Prepares a block template for mining
    pub fn create_block_template(&self, previous_block_hash: &[u8], coinbase_tx: Vec<u8>) -> Block {
        let mut block = Block::new(previous_block_hash.to_vec());

        // Add the coinbase transaction
        block.add_transaction(coinbase_tx);

        // Add transactions from the mempool
        let mempool_transactions = self.mempool.get_transactions();
        for tx in mempool_transactions {
            block.add_transaction(tx);
        }

        block
    }

    /// Mines a block by finding a valid nonce
    pub fn mine_block(&self, block: &mut Block) -> Option<Block> {
        let mut header = block.header.clone();
        let mut nonce: u64 = 0;

        loop {
            header.nonce = nonce;
            let hash = Miner::calculate_hash(&header);

            if check_proof_of_work(&hash, self.difficulty_bits) {
                block.header = header;
                return Some(block.clone());
            }

            nonce += 1;

            if nonce == u64::MAX {
                break; // Failsafe to avoid infinite loops
            }
        }

        None // Mining failed
    }

    /// Submits a mined block to the blockchain
    pub fn submit_block(&self, block: Block) -> bool {
        // Validate the block (e.g., via consensus rules)
        if check_proof_of_work(&Miner::calculate_hash(&block.header), self.difficulty_bits) {
            println!("Block submitted successfully!");
            return true;
        } else {
            println!("Block submission failed!");
            return false;
        }
    }

    /// Calculates the hash of a block header
    fn calculate_hash(header: &BlockHeader) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&header.to_bytes());
        hasher.finalize().to_vec()
    }
}
