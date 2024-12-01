use hmac::{Hmac, Mac};
use sha2::Sha512;

pub fn hmac_sha512(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha512>::new_from_slice(key).expect("Invalid key size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}
