// test_wallet.rs
use crate::wallet::{Wallet, KeyManager, Transaction};
use std::sync::Arc;

// Mock or test helpers
fn setup_mock_wallet() -> Wallet {
    Wallet::new() // Assuming Wallet::new() initializes a mock wallet
}

fn test_generate_new_key() {
    let mut wallet = setup_mock_wallet();

    let new_key = wallet.generate_new_key();
    assert!(new_key.is_some(), "Failed to generate new key");
    let new_key = new_key.unwrap();

    println!("Generated new key: {}", new_key);
}

fn test_send_funds() {
    let mut wallet = setup_mock_wallet();

    // Mock data
    let recipient_address = "mock_recipient_address".to_string();
    let amount = 1000; // Mock amount

    let tx = wallet.send_funds(&recipient_address, amount);
    assert!(tx.is_some(), "Failed to send funds");
    let tx = tx.unwrap();

    println!("Sent transaction: {:?}", tx);
}

fn main() {
    test_generate_new_key();
    test_send_funds();
}
