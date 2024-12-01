#[cfg(test)]
mod tests {
    use crate::miner::generate_block;
    use crate::block::Block;

    #[test]
    fn generate_block() {
        let block = generate_block();
        assert_eq!(block.vtx.len(), 1); // Block contains coinbase transaction
    }
}

pub mod miner {
    use crate::block::Block;

    pub fn generate_block() -> Block {
        // Example: Create a block with a single coinbase transaction
        Block {
            vtx: vec!["coinbase".to_string()],
            n_bits: 0,
            n_nonce: 0,
        }
    }
}

pub mod block {
    #[derive(Default)]
    pub struct Block {
        pub vtx: Vec<String>, // Replace with a proper transaction type
        pub n_bits: u32,
        pub n_nonce: u32,
    }
}
