#[cfg(test)]
mod tests {
    use bitcoin_wallet::wallet::{Wallet, WalletConfig};
    use bitcoin::util::address::Address;
    use bitcoin::blockdata::transaction::Transaction;
    use bitcoin::Network;

    #[test]
    fn test_create_new_address() {
        let wallet_config = WalletConfig {
            network: Network::Bitcoin,
            db_path: "./test_wallet_db".to_string(),
        };
        let mut wallet = Wallet::new(wallet_config).unwrap();

        let new_address = wallet.create_new_address().unwrap();
        assert!(Address::from_str(&new_address).is_ok());
    }

    #[test]
    fn test_send_transaction() {
        let wallet_config = WalletConfig {
            network: Network::Bitcoin,
            db_path: "./test_wallet_db".to_string(),
        };
        let mut wallet = Wallet::new(wallet_config).unwrap();

        let recipient = Address::from_str("1BitcoinAddress...").unwrap();
        let amount = 100_000; // Satoshis
        let tx: Transaction = wallet.create_transaction(&recipient, amount).unwrap();

        assert_eq!(tx.output.len(), 1);
        assert_eq!(tx.output[0].value, amount);
    }

    #[test]
    fn test_manage_utxos() {
        let wallet_config = WalletConfig {
            network: Network::Bitcoin,
            db_path: "./test_wallet_db".to_string(),
        };
        let mut wallet = Wallet::new(wallet_config).unwrap();

        let utxos = wallet.get_utxos().unwrap();
        assert!(utxos.is_empty());
    }
}
