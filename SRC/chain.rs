use std::collections::HashMap;

/// Represents a block header in the blockchain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub hash: String,       // Block hash
    pub prev_hash: String,  // Previous block hash
    pub height: u32,        // Block height
}

/// Represents the active blockchain.
pub struct Blockchain {
    blocks: HashMap<String, BlockHeader>, // Map of block hash to BlockHeader
    active_chain: Vec<BlockHeader>,       // Active chain of blocks
}

impl Blockchain {
    /// Creates a new blockchain.
    pub fn new() -> Self {
        Blockchain {
            blocks: HashMap::new(),
            active_chain: Vec::new(),
        }
    }

    /// Adds a block header to the blockchain.
    pub fn add_block(&mut self, header: BlockHeader) {
        self.blocks.insert(header.hash.clone(), header.clone());
        self.update_active_chain();
    }

    /// Updates the active chain by recalculating the longest chain.
    fn update_active_chain(&mut self) {
        let mut chain = Vec::new();
        if let Some(mut tip) = self.blocks.values().find(|b| self.is_tip(&b.hash)) {
            while let Some(prev) = self.blocks.get(&tip.prev_hash) {
                chain.push(tip.clone());
                tip = prev;
            }
            chain.push(tip.clone());
        }
        chain.reverse();
        self.active_chain = chain;
    }

    /// Checks if a block is a tip of the chain.
    fn is_tip(&self, hash: &String) -> bool {
        !self.blocks.values().any(|b| b.prev_hash == *hash)
    }

    /// Retrieves the block header at the given height in the active chain.
    pub fn get_ancestor(&self, height: u32) -> Option<&BlockHeader> {
        self.active_chain.get(height as usize)
    }

    /// Checks if a block is part of the active chain.
    pub fn contains(&self, hash: &str) -> bool {
        self.active_chain.iter().any(|b| b.hash == hash)
    }

    /// Retrieves the current chain tip.
    pub fn chain_tip(&self) -> Option<&BlockHeader> {
        self.active_chain.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_operations() {
        let mut blockchain = Blockchain::new();

        // Add blocks
        blockchain.add_block(BlockHeader {
            hash: "block1".to_string(),
            prev_hash: "".to_string(),
            height: 0,
        });
        blockchain.add_block(BlockHeader {
            hash: "block2".to_string(),
            prev_hash: "block1".to_string(),
            height: 1,
        });
        blockchain.add_block(BlockHeader {
            hash: "block3".to_string(),
            prev_hash: "block2".to_string(),
            height: 2,
        });

        // Test chain navigation
        assert_eq!(blockchain.chain_tip().unwrap().hash, "block3");
        assert!(blockchain.contains("block2"));
        assert_eq!(
            blockchain.get_ancestor(1).unwrap().hash,
            "block2".to_string()
        );
    }
}
