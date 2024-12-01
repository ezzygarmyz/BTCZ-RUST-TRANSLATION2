use crate::wallet::Wallet;

pub fn get_balance(wallet: &Wallet) -> u64 {
    wallet.get_balance()
}

pub fn send_to_address(wallet: &mut Wallet, to_address: &str, amount: u64, fee: u64) -> bool {
    if let Some(tx) = wallet.create_transaction(to_address, amount, fee) {
        println!("Transaction sent: {:?}", tx);
        true
    } else {
        false
    }
}
