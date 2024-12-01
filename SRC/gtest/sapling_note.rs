#[cfg(test)]
mod tests {
    use crate::sapling_note::SaplingNote;

    #[test]
    fn generate_note() {
        let note = SaplingNote::new();
        assert!(note.is_valid());
    }
}

pub mod sapling_note {
    pub struct SaplingNote {
        valid: bool,
    }

    impl SaplingNote {
        pub fn new() -> Self {
            SaplingNote { valid: true }
        }

        pub fn is_valid(&self) -> bool {
            self.valid
        }
    }
}
