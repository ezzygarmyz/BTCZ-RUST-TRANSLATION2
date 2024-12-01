use secp256k1::{
    All, Message, PublicKey, Secp256k1, SecretKey, Signature,
};
use rand::rngs::OsRng;

/// Represents a private key.
pub struct Key {
    secret_key: SecretKey,
    secp: Secp256k1<All>,
}

impl Key {
    /// Generates a new random private key.
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::new(&mut OsRng);
        Key { secret_key, secp }
    }

    /// Derives the public key from the private key.
    pub fn get_pub_key(&self) -> PublicKey {
        PublicKey::from_secret_key(&self.secp, &self.secret_key)
    }

    /// Signs a message hash using the private key.
    pub fn sign(&self, hash: &[u8]) -> Option<Signature> {
        let message = Message::from_slice(hash).ok()?;
        Some(self.secp.sign_ecdsa(&message, &self.secret_key))
    }

    /// Verifies a signature using the corresponding public key.
    pub fn verify(&self, hash: &[u8], signature: &Signature, public_key: &PublicKey) -> bool {
        let message = Message::from_slice(hash).ok();
        match message {
            Some(msg) => self.secp.verify_ecdsa(&msg, signature, public_key).is_ok(),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_key_operations() {
        let key = Key::new();
        let pub_key = key.get_pub_key();

        // Example hash
        let hash = hex::decode("b10c0adc8a1e2ffea57c6e4e2e88c8b7f9a6f59c8a1e2f74e69c902d").unwrap();

        // Sign the hash
        let signature = key.sign(&hash).expect("Signing failed");

        // Verify the signature
        assert!(key.verify(&hash, &signature, &pub_key));
    }
}
