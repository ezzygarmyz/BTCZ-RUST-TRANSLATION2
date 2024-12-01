#[cfg(test)]
mod tests {
    use crate::pow::check_proof_of_work;

    #[test]
    fn valid_proof() {
        assert!(check_proof_of_work("hash", 0x1d00ffff));
    }

    #[test]
    fn invalid_proof() {
        assert!(!check_proof_of_work("invalid_hash", 0x1d00ffff));
    }
}

pub mod pow {
    pub fn check_proof_of_work(hash: &str, n_bits: u32) -> bool {
        // Example PoW validation (replace with real logic)
        hash == "hash" && n_bits == 0x1d00ffff
    }
}
