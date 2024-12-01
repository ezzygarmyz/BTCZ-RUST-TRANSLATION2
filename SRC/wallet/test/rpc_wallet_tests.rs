#[cfg(test)]
mod tests {
    use bitcoin_wallet::wallet::{Wallet, WalletConfig};
    use bitcoin_wallet::rpc::{RpcClient, RpcError};

    #[test]
    fn test_create_wallet() {
        let wallet_config = WalletConfig {
            network: bitcoin::Network::Bitcoin,
            db_path: "./test_wallet_db".to_string(),
        };
        let wallet = Wallet::new(wallet_config).unwrap();

        assert!(wallet.is_empty());
    }

    #[test]
    fn test_rpc_get_balance() {
        let rpc_client = RpcClient::new("http://localhost:8332", "user", "password").unwrap();
        match rpc_client.get_balance() {
            Ok(balance) => assert!(balance >= 0.0),
            Err(err) => panic!("RPC call failed: {:?}", err),
        }
    }
}
