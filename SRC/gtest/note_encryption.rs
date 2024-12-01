#[cfg(test)]
mod tests {
    use crate::note_encryption::{encrypt_note, decrypt_note};

    #[test]
    fn encrypt_decrypt() {
        let plaintext = "secret";
        let ciphertext = encrypt_note(plaintext);
        let decrypted = decrypt_note(&ciphertext);
        assert_eq!(decrypted, plaintext);
    }
}

pub mod note_encryption {
    pub fn encrypt_note(plaintext: &str) -> Vec<u8> {
        // Simple mock encryption (replace with actual implementation)
        plaintext.as_bytes().to_vec()
    }

    pub fn decrypt_note(ciphertext: &[u8]) -> String {
        // Simple mock decryption (replace with actual implementation)
        String::from_utf8(ciphertext.to_vec()).unwrap()
    }
}
