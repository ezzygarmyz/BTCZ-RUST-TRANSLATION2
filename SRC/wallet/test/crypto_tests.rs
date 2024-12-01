#[cfg(test)]
mod tests {
    use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
    use bitcoin::util::bip39::{Mnemonic, Language};
    use bitcoin::secp256k1::{Secp256k1, SecretKey};

    #[test]
    fn test_key_derivation() {
        let mnemonic = Mnemonic::new("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about", Language::English).unwrap();
        let seed = mnemonic.to_seed("");
        let secp = Secp256k1::new();
        let xpriv = ExtendedPrivKey::new_master(bitcoin::Network::Bitcoin, &seed).unwrap();

        let derived_key = xpriv.derive_priv(&secp, &[0, 1, 2]).unwrap();
        assert_eq!(derived_key.depth, 3);
    }

    #[test]
    fn test_key_encryption() {
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&[0xcd; 32]).unwrap();
        let serialized_sk = sk.secret_bytes();

        // Simulating encryption/decryption
        let encrypted = serialized_sk.iter().map(|b| b ^ 0xff).collect::<Vec<u8>>();
        let decrypted = encrypted.iter().map(|b| b ^ 0xff).collect::<Vec<u8>>();

        assert_eq!(serialized_sk.to_vec(), decrypted);
    }
}
