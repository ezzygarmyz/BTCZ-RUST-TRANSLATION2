pub struct WalletTestFixture {
    pub wallet_path: String,
}

impl WalletTestFixture {
    pub fn new() -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        WalletTestFixture {
            wallet_path: temp_dir.path().to_str().unwrap().to_string(),
        }
    }

    pub fn setup_wallet(&self) -> bitcoin_wallet::wallet::Wallet {
        let wallet_config = bitcoin_wallet::wallet::WalletConfig {
            network: bitcoin::Network::Bitcoin,
            db_path: self.wallet_path.clone(),
        };
        bitcoin_wallet::wallet::Wallet::new(wallet_config).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_fixture() {
        let fixture = WalletTestFixture::new();
        let wallet = fixture.setup_wallet();

        assert!(wallet.is_empty());
    }
}
