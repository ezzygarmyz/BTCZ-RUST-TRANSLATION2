#[cfg(test)]
mod tests {
    use crate::validation::check_block;
    use crate::block::Block;

    #[test]
    fn check_block() {
        let block = Block::default();
        assert!(check_block(&block));
    }

    #[test]
    fn check_invalid_block() {
        let mut block = Block::default();
        block.n_bits = 0; // Invalid block
        assert!(!check_block(&block));
    }
}

pub mod validation {
    use crate::block::Block;

    pub fn check_block(block: &Block) -> bool {
        block.n_bits > 0 && !block.vtx.is_empty()
    }
}

pub mod block {
    #[derive(Default)]
    pub struct Block {
        pub vtx: Vec<String>, // Replace with actual transaction type
        pub n_bits: u32,
    }
}
