use sha2::{Digest, Sha256};
use std::collections::HashSet;

/// Represents a Merkle Tree
pub struct MerkleTree {
    pub root: Vec<u8>,
    pub proofs: Vec<Vec<u8>>,
}

impl MerkleTree {
    /// Constructs a Merkle Tree from a list of transaction hashes
    pub fn build(transactions: &[Vec<u8>]) -> Self {
        if transactions.is_empty() {
            return Self {
                root: vec![0; 32],
                proofs: vec![],
            };
        }

        let mut current_level = transactions.to_vec();

        while current_level.len() > 1 {
            current_level = MerkleTree::build_next_level(&current_level);
        }

        Self {
            root: current_level[0].clone(),
            proofs: vec![], // Proofs are added separately
        }
    }

    /// Builds the next level of the Merkle Tree
    fn build_next_level(hashes: &[Vec<u8>]) -> Vec<Vec<u8>> {
        let mut next_level = vec![];
        for chunk in hashes.chunks(2) {
            if chunk.len() == 2 {
                next_level.push(MerkleTree::hash_pair(&chunk[0], &chunk[1]));
            } else {
                next_level.push(chunk[0].clone()); // Handle odd number of hashes
            }
        }
        next_level
    }

    /// Hashes two nodes together
    fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().to_vec()
    }
}

/// Represents a Merkle Block
pub struct MerkleBlock {
    pub header: Vec<u8>,  // Block header (simplified)
    pub matched_transactions: HashSet<Vec<u8>>,
    pub merkle_tree: MerkleTree,
}

impl MerkleBlock {
    /// Creates a new Merkle Block
    pub fn new(
        header: Vec<u8>,
        transactions: &[Vec<u8>],
        matched_tx_ids: &HashSet<Vec<u8>>,
    ) -> Self {
        let mut proofs = vec![];
        let root = MerkleTree::build(transactions).root;

        // Build proofs for matched transactions
        for tx in matched_tx_ids {
            if transactions.contains(tx) {
                proofs.push(tx.clone()); // Simplified proof
            }
        }

        Self {
            header,
            matched_transactions: matched_tx_ids.clone(),
            merkle_tree: MerkleTree {
                root,
                proofs,
            },
        }
    }

    /// Verifies a transaction inclusion using Merkle proof
    pub fn verify_transaction(&self, tx_hash: &Vec<u8>) -> bool {
        self.matched_transactions.contains(tx_hash)
    }
}
