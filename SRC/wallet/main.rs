mod wallet;
mod keys;
mod utxo;
mod transactions;
mod backup;
mod rpc;

use wallet::Wallet;

fn main() {
    let mut wallet = Wallet::new();
    wallet.add_utxo("txid1".to_string(), 50000);

    println!("Balance: {}", rpc::get_balance(&wallet));
    rpc::send_to_address(&mut wallet, "BTCZAddress123", 10000, 500);
}
