use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a network address.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkAddress {
    pub ip: String,
    pub port: u16,
}

/// Represents a peer entry with scoring.
#[derive(Debug, Clone)]
pub struct PeerEntry {
    pub address: NetworkAddress,
    pub source: NetworkAddress,
    pub last_seen: u64,
    pub score: i32,
}

/// Manages a list of peers and scoring.
pub struct AddressManager {
    peers: HashMap<NetworkAddress, PeerEntry>,
}

impl AddressManager {
    /// Creates a new AddressManager.
    pub fn new() -> Self {
        AddressManager {
            peers: HashMap::new(),
        }
    }

    /// Adds a new peer to the manager.
    pub fn add_peer(
        &mut self,
        address: NetworkAddress,
        source: NetworkAddress,
        time_penalty: i64,
    ) {
        let now = current_timestamp();
        let entry = PeerEntry {
            address: address.clone(),
            source,
            last_seen: now - time_penalty as u64,
            score: 0,
        };
        self.peers.insert(address, entry);
    }

    /// Selects the best peer based on scoring.
    pub fn select_peer(&self) -> Option<&PeerEntry> {
        let mut candidates: Vec<&PeerEntry> = self.peers.values().collect();
        candidates.sort_by(|a, b| b.score.cmp(&a.score)); // Higher score = better peer
        candidates.first().cloned()
    }

    /// Marks a peer as good by increasing its score.
    pub fn mark_good(&mut self, address: &NetworkAddress) {
        if let Some(entry) = self.peers.get_mut(address) {
            entry.score += 10;
        }
    }

    /// Marks a peer as bad by decreasing its score.
    pub fn mark_bad(&mut self, address: &NetworkAddress) {
        if let Some(entry) = self.peers.get_mut(address) {
            entry.score -= 10;
        }
    }

    /// Removes a peer from the manager.
    pub fn remove_peer(&mut self, address: &NetworkAddress) {
        self.peers.remove(address);
    }
}

/// Returns the current timestamp in seconds since the UNIX epoch.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_manager() {
        let mut addr_manager = AddressManager::new();

        let addr1 = NetworkAddress {
            ip: "192.168.1.1".to_string(),
            port: 8333,
        };
        let addr2 = NetworkAddress {
            ip: "10.0.0.1".to_string(),
            port: 8333,
        };

        let source = NetworkAddress {
            ip: "127.0.0.1".to_string(),
            port: 8333,
        };

        addr_manager.add_peer(addr1.clone(), source.clone(), 0);
        addr_manager.add_peer(addr2.clone(), source.clone(), 10);

        assert_eq!(addr_manager.peers.len(), 2);

        addr_manager.mark_good(&addr1);
        let selected = addr_manager.select_peer();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().address, addr1);

        addr_manager.mark_bad(&addr1);
        addr_manager.mark_bad(&addr2);
        addr_manager.remove_peer(&addr2);
        assert_eq!(addr_manager.peers.len(), 1);
    }
}
