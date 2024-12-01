/// DNS seeds for the mainnet.
pub const MAINNET_DNS_SEEDS: &[&str] = &[
    "dnsseed.btcz.rocks",
    "dnsseed.btcz.org",
    "btcz.seeds.network",
];

/// DNS seeds for the testnet.
pub const TESTNET_DNS_SEEDS: &[&str] = &[
    "testnet-seed.btcz.rocks",
    "testnet-seed.btcz.org",
];

/// Hardcoded IP addresses for mainnet.
pub const MAINNET_FIXED_SEEDS: &[&str] = &[
    "192.168.1.1",
    "192.168.1.2",
];

/// Hardcoded IP addresses for testnet.
pub const TESTNET_FIXED_SEEDS: &[&str] = &[
    "192.168.2.1",
    "192.168.2.2",
];

/// Returns the DNS seeds for the specified network.
pub fn get_dns_seeds(network: &str) -> Option<&[&str]> {
    match network {
        "mainnet" => Some(MAINNET_DNS_SEEDS),
        "testnet" => Some(TESTNET_DNS_SEEDS),
        _ => None,
    }
}

/// Returns the fixed seeds for the specified network.
pub fn get_fixed_seeds(network: &str) -> Option<&[&str]> {
    match network {
        "mainnet" => Some(MAINNET_FIXED_SEEDS),
        "testnet" => Some(TESTNET_FIXED_SEEDS),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_seeds() {
        let mainnet_seeds = get_dns_seeds("mainnet").unwrap();
        assert_eq!(mainnet_seeds.len(), 3);
        assert_eq!(mainnet_seeds[0], "dnsseed.btcz.rocks");

        let testnet_seeds = get_dns_seeds("testnet").unwrap();
        assert_eq!(testnet_seeds.len(), 2);
        assert_eq!(testnet_seeds[0], "testnet-seed.btcz.rocks");

        assert!(get_dns_seeds("invalid").is_none());
    }

    #[test]
    fn test_fixed_seeds() {
        let mainnet_seeds = get_fixed_seeds("mainnet").unwrap();
        assert_eq!(mainnet_seeds.len(), 2);
        assert_eq!(mainnet_seeds[0], "192.168.1.1");

        let testnet_seeds = get_fixed_seeds("testnet").unwrap();
        assert_eq!(testnet_seeds.len(), 2);
        assert_eq!(testnet_seeds[0], "192.168.2.1");

        assert!(get_fixed_seeds("invalid").is_none());
    }
}
