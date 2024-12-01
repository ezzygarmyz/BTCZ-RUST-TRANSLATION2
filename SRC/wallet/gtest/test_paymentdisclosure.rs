// test_payment_disclosure.rs
use crate::wallet::{Wallet, PaymentDisclosure};
use std::sync::Arc;

// Mock or test helpers
fn setup_mock_wallet() -> Wallet {
    Wallet::new() // Assuming Wallet::new() initializes a mock wallet
}

fn test_create_payment_disclosure() {
    let wallet = setup_mock_wallet();

    // Mock data
    let tx_id = "mock_tx_id".to_string();
    let output_index = 0;
    let shared_secret = "mock_shared_secret".to_string();

    let disclosure = wallet.create_payment_disclosure(&tx_id, output_index, &shared_secret);
    assert!(disclosure.is_some(), "Failed to create payment disclosure");
    let disclosure = disclosure.unwrap();

    // Validate the disclosure details
    assert_eq!(disclosure.tx_id, tx_id, "Transaction ID mismatch");
    assert_eq!(disclosure.output_index, output_index, "Output index mismatch");
    assert_eq!(
        disclosure.shared_secret, shared_secret,
        "Shared secret mismatch"
    );
    println!("Payment disclosure test passed!");
}

fn main() {
    test_create_payment_disclosure();
}
