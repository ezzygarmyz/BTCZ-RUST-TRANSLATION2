use ripemd160::{Digest, Ripemd160};

pub fn ripemd160_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
