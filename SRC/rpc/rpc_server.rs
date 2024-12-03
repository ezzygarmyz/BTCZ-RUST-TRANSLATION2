use crate::rpc::{RpcRegistry, RpcRequest, RpcResponse, RpcError};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;

/// JSON-RPC server implementation
pub struct RpcServer {
    registry: Arc<RpcRegistry>,
    address: String,
}

impl RpcServer {
    /// Creates a new RPC server
    pub fn new(address: &str, registry: Arc<RpcRegistry>) -> Self {
        RpcServer {
            registry,
            address: address.to_string(),
        }
    }

    /// Starts the RPC server
    pub async fn start(&self) -> Result<(), RpcError> {
        let listener = TcpListener::bind(&self.address).await.map_err(|e| {
            RpcError::internal_error(format!("Failed to bind address {}: {}", self.address, e))
        })?;

        println!("RPC server listening on {}", self.address);

        loop {
            let (mut socket, _) = listener.accept().await.map_err(|e| {
                RpcError::internal_error(format!("Failed to accept connection: {}", e))
            })?;

            let registry = Arc::clone(&self.registry);

            tokio::spawn(async move {
                let mut buffer = Vec::new();
                if let Ok(_) = socket.read_to_end(&mut buffer).await {
                    let response = RpcServer::handle_request(&registry, buffer).await;
                    if let Ok(response_data) = response {
                        let _ = socket.write_all(&response_data).await;
                    }
                }
            });
        }
    }

    /// Handles a single RPC request
    async fn handle_request(
        registry: &Arc<RpcRegistry>,
        request_data: Vec<u8>,
    ) -> Result<Vec<u8>, RpcError> {
        let request_str = String::from_utf8_lossy(&request_data);
        let request_json: Value = serde_json::from_str(&request_str).map_err(|e| {
            RpcError::internal_error(format!("Failed to parse request: {}", e))
        })?;

        let rpc_request = RpcRequest::from_json(&request_json).map_err(|e| {
            RpcError::internal_error(format!("Invalid RPC request format: {}", e))
        })?;

        let rpc_response = registry.dispatch(rpc_request);
        let response_json = rpc_response.to_json();

        serde_json::to_vec(&response_json).map_err(|e| {
            RpcError::internal_error(format!("Failed to serialize response: {}", e))
        })
    }
}
