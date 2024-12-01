use std::fs;
use crate::wallet::Wallet;

pub fn backup_wallet(wallet: &Wallet, filename: &str) -> Result<(), String> {
    let serialized = serde_json::to_string(wallet).map_err(|_| "Serialization failed".to_string())?;
    fs::write(filename, serialized).map_err(|_| "Failed to write file".to_string())
}

pub fn restore_wallet(filename: &str) -> Result<Wallet, String> {
    let data = fs::read_to_string(filename).map_err(|_| "Failed to read file".to_string())?;
    serde_json::from_str(&data).map_err(|_| "Deserialization failed".to_string())
}
