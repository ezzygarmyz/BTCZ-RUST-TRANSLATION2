#[cfg(test)]
mod tests {
    use crate::libzcash_utils::compute_hash;

    #[test]
    fn compute_hash() {
        let input = "data";
        let result = compute_hash(input);
        assert_eq!(result, "expected_hash");
    }
}

pub mod libzcash_utils {
    use sha2::{Digest, Sha256};

    pub fn compute_hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}
