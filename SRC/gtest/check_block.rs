#[cfg(test)]
mod tests {
    use crate::consensus::validation::check_block;
    use crate::primitives::block::Block;

    #[test]
    fn valid_block() {
        let block = Block::default();
        assert!(check_block(&block));
    }
}

pub mod consensus {
    pub mod validation {
        use crate::primitives::block::Block;

        pub fn check_block(block: &Block) -> bool {
            // Implement block validation logic
            true
        }
    }
}
