#[cfg(test)]
mod tests {
    use crate::keystore::{Key, KeyStore};

    #[test]
    fn add_key() {
        let key = Key::new(true);
        let mut keystore = KeyStore::new();
        keystore.add_key(&key);
        assert!(keystore.have_key(&key.get_pub_key()));
    }
}

pub mod keystore {
    use crate::keys::Key;

    pub struct KeyStore {
        keys: Vec<String>, // Example public key storage
    }

    impl KeyStore {
        pub fn new() -> Self {
            KeyStore { keys: Vec::new() }
        }

        pub fn add_key(&mut self, key: &Key) {
            self.keys.push(key.get_pub_key());
        }

        pub fn have_key(&self, pub_key: &str) -> bool {
            self.keys.contains(pub_key)
        }
    }
}

pub mod keys {
    pub struct Key {
        pub valid: bool,
    }

    impl Key {
        pub fn new(valid: bool) -> Self {
            Key { valid }
        }

        pub fn get_pub_key(&self) -> String {
            // Mock public key
            "pub_key".to_string()
        }
    }
}
