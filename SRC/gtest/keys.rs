#[cfg(test)]
mod tests {
    use crate::keys::Key;

    #[test]
    fn valid_key() {
        let key = Key::new(true);
        assert!(key.is_valid());
    }
}

pub mod keys {
    pub struct Key {
        valid: bool,
    }

    impl Key {
        pub fn new(valid: bool) -> Self {
            Key { valid }
        }

        pub fn is_valid(&self) -> bool {
            self.valid
        }
    }
}
