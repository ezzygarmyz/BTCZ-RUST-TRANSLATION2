use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Represents a peer in the network.
pub struct Peer {
    pub id: u64,
    pub address: String,
}

/// Represents a message received from a peer.
#[derive(Deserialize)]
pub struct PeerMessage {
    pub command: String,
    pub payload: serde_json::Value,
}

/// Handles the processing of messages from peers.
pub struct NetProcessor {
    peers: Arc<RwLock<HashMap<u64, Peer>>>,
}

impl NetProcessor {
    /// Creates a new NetProcessor instance.
    pub fn new() -> Self {
        NetProcessor {
            peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Processes an incoming message from a peer.
    pub async fn process_message(&self, peer_id: u64, message: PeerMessage) {
        match message.command.as_str() {
            "tx" => {
                self.handle_transaction(peer_id, message.payload).await;
            }
            "block" => {
                self.handle_block(peer_id, message.payload).await;
            }
            _ => {
                eprintln!("Unknown command: {}", message.command);
            }
        }
    }

    /// Handles a "tx" (transaction) message.
    async fn handle_transaction(&self, peer_id: u64, payload: serde_json::Value) {
        println!(
            "Processing transaction from peer {}: {:?}",
            peer_id, payload
        );
        // Add validation and relay logic here
    }

    /// Handles a "block" (block announcement) message.
    async fn handle_block(&self, peer_id: u64, payload: serde_json::Value) {
        println!("Processing block from peer {}: {:?}", peer_id, payload);
        // Add validation and block request logic here
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_message_processing() {
        let net_processor = NetProcessor::new();

        let peer_message = PeerMessage {
            command: "tx".to_string(),
            payload: json!({"txid": "abcd1234", "amount": 100}),
        };

        net_processor.process_message(1, peer_message).await;

        let peer_message = PeerMessage {
            command: "block".to_string(),
            payload: json!({"blockhash": "1234abcd", "height": 100}),
        };

        net_processor.process_message(1, peer_message).await;

        let peer_message = PeerMessage {
            command: "unknown".to_string(),
            payload: json!({}),
        };

        net_processor.process_message(1, peer_message).await;
    }
}
