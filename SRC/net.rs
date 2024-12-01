use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Represents a peer in the network.
#[derive(Debug, Clone)]
pub struct Peer {
    pub address: String,
    pub stream: TcpStream,
}

/// Represents a protocol message.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub command: String,
    pub payload: String,
}

/// Manages the network state.
pub struct Network {
    peers: Arc<RwLock<HashMap<String, Peer>>>,
    sender: mpsc::Sender<Message>,
}

impl Network {
    /// Creates a new network manager.
    pub fn new() -> (Self, mpsc::Receiver<Message>) {
        let (sender, receiver) = mpsc::channel(100);
        (
            Network {
                peers: Arc::new(RwLock::new(HashMap::new())),
                sender,
            },
            receiver,
        )
    }

    /// Adds a peer to the network.
    pub async fn add_peer(&self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(address).await?;
        let peer = Peer {
            address: address.to_string(),
            stream,
        };
        self.peers.write().await.insert(address.to_string(), peer);
        Ok(())
    }

    /// Sends a message to a peer.
    pub async fn send_message(&self, address: &str, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let peers = self.peers.read().await;
        if let Some(peer) = peers.get(address) {
            let mut stream = &peer.stream;
            let message_data = serde_json::to_vec(message)?;
            stream.write_all(&message_data).await?;
        }
        Ok(())
    }

    /// Processes incoming messages from peers.
    pub async fn process_messages(&self) -> Result<(), Box<dyn std::error::Error>> {
        let peers = self.peers.read().await;

        for peer in peers.values() {
            let mut buffer = vec![0; 1024];
            let mut stream = &peer.stream;

            let bytes_read = stream.read(&mut buffer).await?;
            if bytes_read > 0 {
                let message: Message = serde_json::from_slice(&buffer[..bytes_read])?;
                self.sender.send(message).await.unwrap();
            }
        }
        Ok(())
    }
}

/// Starts the network event loop.
pub async fn network_event_loop(network: Arc<Network>) {
    loop {
        if let Err(e) = network.process_messages().await {
            eprintln!("Error processing messages: {}", e);
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_peer_connection() {
        let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
        tokio::spawn(async move {
            let (_socket, _addr) = listener.accept().await.unwrap();
        });

        let (network, _receiver) = Network::new();
        network.add_peer("127.0.0.1:12345").await.unwrap();

        let peers = network.peers.read().await;
        assert!(peers.contains_key("127.0.0.1:12345"));
    }

    #[tokio::test]
    async fn test_message_sending() {
        let listener = TcpListener::bind("127.0.0.1:12346").await.unwrap();
        tokio::spawn(async move {
            let (mut socket, _addr) = listener.accept().await.unwrap();
            let mut buffer = vec![0; 1024];
            let bytes_read = socket.read(&mut buffer).await.unwrap();
            let message: Message = serde_json::from_slice(&buffer[..bytes_read]).unwrap();
            assert_eq!(message.command, "ping");
        });

        let (network, _receiver) = Network::new();
        network.add_peer("127.0.0.1:12346").await.unwrap();

        let message = Message {
            command: "ping".to_string(),
            payload: "".to_string(),
        };

        network
            .send_message("127.0.0.1:12346", &message)
            .await
            .unwrap();
    }
}
