use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use serde_json::{Value, json};
use thiserror::Error;

/// Custom error type for RPC client operations
#[derive(Debug, Error)]
pub enum RpcClientError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("RPC error: {0}")]
    Rpc(String),
}

/// Represents an RPC client for communicating with the BTCZ server
pub struct RpcClient {
    address: String,
    timeout: Duration,
}

impl RpcClient {
    /// Creates a new RPC client with the specified server address and timeout
    pub fn new(address: &str, timeout: Duration) -> Self {
        RpcClient {
            address: address.to_string(),
            timeout,
        }
    }

    /// Sends an RPC request with the given method and parameters
    pub fn send_request(&self, method: &str, params: &[Value]) -> Result<Value, RpcClientError> {
        // Construct the JSON-RPC request payload
        let request = json!({
            "jsonrpc": "1.0",
            "id": "btcz",
            "method": method,
            "params": params,
        });

        // Serialize the request to a JSON string
        let request_str = serde_json::to_string(&request)?;

        // Establish a TCP connection to the server
        let mut stream = TcpStream::connect(&self.address)?;
        stream.set_read_timeout(Some(self.timeout))?;
        stream.set_write_timeout(Some(self.timeout))?;

        // Send the request
        stream.write_all(request_str.as_bytes())?;
        stream.write_all(b"\n")?; // Ensure the request is newline-terminated

        // Read the response
        let mut response_str = String::new();
        stream.read_to_string(&mut response_str)?;

        // Deserialize the response
        let response: Value = serde_json::from_str(&response_str)?;

        // Check for RPC errors
        if let Some(error) = response.get("error") {
            if !error.is_null() {
                return Err(RpcClientError::Rpc(error.to_string()));
            }
        }

        // Return the result
        Ok(response["result"].clone())
    }
}
