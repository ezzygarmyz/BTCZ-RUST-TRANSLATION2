use secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin_hashes::{sha256, Hash};

pub struct KeyPair {
    private_key: SecretKey,
    public_key: PublicKey,
}

impl KeyPair {
    /// Generate a new random key pair
    pub fn generate() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        KeyPair {
            private_key: secret_key,
            public_key,
        }
    }

    /// Derive the wallet address from the public key
    pub fn get_address(&self) -> String {
        let pubkey_hash = sha256::Hash::hash(&self.public_key.serialize());
        format!("BTCZ{}", hex::encode(pubkey_hash))
    }

    /// Get the private key
    pub fn get_private_key(&self) -> String {
        hex::encode(self.private_key.secret_bytes())
    }
}
