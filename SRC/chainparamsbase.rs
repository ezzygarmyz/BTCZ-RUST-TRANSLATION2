use std::str::FromStr;

/// Represents the base parameters for a blockchain network.
#[derive(Debug, Clone)]
pub struct BaseChainParams {
    pub network: NetworkType,
    pub rpc_port: u16,
    pub data_dir: String,
}

/// Enumeration of network types.
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkType {
    Main,
    Test,
    RegTest,
}

impl FromStr for NetworkType {
    type Err = String;

    fn from_str(network: &str) -> Result<Self, Self::Err> {
        match network {
            "main" => Ok(NetworkType::Main),
            "test" => Ok(NetworkType::Test),
            "regtest" => Ok(NetworkType::RegTest),
            _ => Err(format!("Invalid network type: {}", network)),
        }
    }
}

impl BaseChainParams {
    /// Creates a new BaseChainParams based on the network type.
    pub fn new(network: &str) -> Result<Self, String> {
        let network_type = NetworkType::from_str(network)?;

        let (rpc_port, data_dir) = match network_type {
            NetworkType::Main => (8232, "mainnet".to_string()),
            NetworkType::Test => (18232, "testnet".to_string()),
            NetworkType::RegTest => (18444, "regtest".to_string()),
        };

        Ok(BaseChainParams {
            network: network_type,
            rpc_port,
            data_dir,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_chain_params() {
        let main_params = BaseChainParams::new("main").unwrap();
        assert_eq!(main_params.network, NetworkType::Main);
        assert_eq!(main_params.rpc_port, 8232);
        assert_eq!(main_params.data_dir, "mainnet");

        let test_params = BaseChainParams::new("test").unwrap();
        assert_eq!(test_params.network, NetworkType::Test);
        assert_eq!(test_params.rpc_port, 18232);
        assert_eq!(test_params.data_dir, "testnet");

        let regtest_params = BaseChainParams::new("regtest").unwrap();
        assert_eq!(regtest_params.network, NetworkType::RegTest);
        assert_eq!(regtest_params.rpc_port, 18444);
        assert_eq!(regtest_params.data_dir, "regtest");

        assert!(BaseChainParams::new("invalid").is_err());
    }
}
