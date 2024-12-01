#[cfg(test)]
mod tests {
    use crate::primitives::block::Block;

    #[test]
    fn default_constructor() {
        let block = Block::default();
        assert_eq!(block.vtx.len(), 0);
        assert_eq!(block.n_bits, 0);
        assert_eq!(block.n_nonce, 0);
    }
}

pub mod primitives {
    pub mod block {
        #[derive(Default)]
        pub struct Block {
            pub vtx: Vec<String>, // Replace `String` with actual transaction type
            pub n_bits: u32,
            pub n_nonce: u32,
        }
    }
}
