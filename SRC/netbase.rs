use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};
use std::str::FromStr;

/// Resolves a domain name to an IP address.
pub fn lookup(name: &str) -> Result<IpAddr, String> {
    match name.to_socket_addrs() {
        Ok(mut addrs) => addrs
            .next()
            .map(|addr| addr.ip())
            .ok_or_else(|| "No IP address found".to_string()),
        Err(_) => Err("Failed to resolve domain name".to_string()),
    }
}

/// Validates whether a string is a valid IP address.
pub fn is_valid_ip_address(ip: &str) -> bool {
    IpAddr::from_str(ip).is_ok()
}

/// Converts a hostname to a canonical form (example: lowercased).
pub fn canonicalize_hostname(hostname: &str) -> String {
    hostname.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        // Resolve a known hostname
        match lookup("localhost") {
            Ok(ip) => assert!(ip.is_loopback()),
            Err(_) => panic!("Failed to resolve localhost"),
        }

        // Try resolving an invalid hostna
