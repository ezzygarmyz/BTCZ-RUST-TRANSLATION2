use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit, generic_array::GenericArray};
use aes::{Aes128, Aes192, Aes256};

pub enum AesKey {
    Aes128(Aes128),
    Aes192(Aes192),
    Aes256(Aes256),
}

impl AesKey {
    pub fn new(key: &[u8]) -> Result<Self, String> {
        match key.len() {
            16 => Ok(Self::Aes128(Aes128::new(GenericArray::from_slice(key)))),
            24 => Ok(Self::Aes192(Aes192::new(GenericArray::from_slice(key)))),
            32 => Ok(Self::Aes256(Aes256::new(GenericArray::from_slice(key)))),
            _ => Err("Invalid key size".to_string()),
        }
    }

    pub fn encrypt(&self, block: &mut [u8; 16]) {
        let block_array = GenericArray::from_mut_slice(block);
        match self {
            AesKey::Aes128(key) => key.encrypt_block(block_array),
            AesKey::Aes192(key) => key.encrypt_block(block_array),
            AesKey::Aes256(key) => key.encrypt_block(block_array),
        }
    }

    pub fn decrypt(&self, block: &mut [u8; 16]) {
        let block_array = GenericArray::from_mut_slice(block);
        match self {
            AesKey::Aes128(key) => key.decrypt_block(block_array),
            AesKey::Aes192(key) => key.decrypt_block(block_array),
            AesKey::Aes256(key) => key.decrypt_block(block_array),
        }
    }
}
