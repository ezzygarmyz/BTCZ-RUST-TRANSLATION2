use std::string::FromUtf8Error;
use hex::{encode as hex_encode, decode as hex_decode};
use base64::{encode as base64_encode, decode as base64_decode};
use thiserror::Error;

/// Errors related to string encoding and decoding
#[derive(Debug, Error)]
pub enum EncodingError {
    #[error("Hex decoding failed")]
    HexDecodeError(#[from] hex::FromHexError),
    #[error("Base64 decoding failed")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("UTF-8 decoding failed")]
    Utf8Error(#[from] FromUtf8Error),
}

/// Encodes a byte slice into a hex string
pub fn encode_hex(data: &[u8]) -> String {
    hex_encode(data)
}

/// Decodes a hex string into bytes
pub fn decode_hex(hex: &str) -> Result<Vec<u8>, EncodingError> {
    Ok(hex_decode(hex)?)
}

/// Encodes a byte slice into a base64 string
pub fn encode_base64(data: &[u8]) -> String {
    base64_encode(data)
}

/// Decodes a base64 string into bytes
pub fn decode_base64(base64: &str) -> Result<Vec<u8>, EncodingError> {
    Ok(base64_decode(base64)?)
}

/// Validates if a string is valid hex
pub fn is_valid_hex(hex: &str) -> bool {
    hex_decode(hex).is_ok()
}

/// Validates if a string is valid base64
pub fn is_valid_base64(base64: &str) -> bool {
    base64_decode(base64).is_ok()
}
