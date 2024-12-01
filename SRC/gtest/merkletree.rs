#[cfg(test)]
mod tests {
    use crate::merkletree::compute_merkle_root;

    #[test]
    fn compute_root() {
        let transactions = vec!["tx1".to_string(), "tx2".to_string(), "tx3".to_string(), "tx4".to_string()];
        let root = compute_merkle_root(&transactions);
        assert_eq!(root, "expected_merkle_root");
    }
}

pub mod merkletree {
    use sha2::{Digest, Sha256};

    pub fn compute_merkle_root(transactions: &[String]) -> String {
        if transactions.is_empty() {
            return "empty_tree".to_string();
        }

        let mut hashes: Vec<String> = transactions.iter().map(|tx| hash(tx)).collect();

        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            for pair in hashes.chunks(2) {
                let concatenated = pair.join("");
                new_hashes.push(hash(&concatenated));
            }
            hashes = new_hashes;
        }

        hashes[0].clone()
    }

    fn hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}
