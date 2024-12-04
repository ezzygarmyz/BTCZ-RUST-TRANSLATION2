use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use serde_json::{json, Value};
use std::collections::HashMap;
use thiserror::Error;

/// Custom error type for RPC handling
#[derive(Debug, Error)]
pub enum RpcError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("RPC error: {0}")]
    Rpc(String),
}

/// Struct for RPC configuration
#[derive(Debug)]
pub struct RpcConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

/// Sends an RPC command to the BitcoinZ daemon
pub fn send_rpc_command(
    config: &RpcConfig,
    method: &str,
    params: &[Value],
) -> Result<Value, RpcError> {
    let request = json!({
        "jsonrpc": "1.0",
        "id": "btcz",
        "method": method,
        "params": params,
    });

    let mut stream = TcpStream::connect(format!("{}:{}", config.host, config.port))?;
    let auth = base64::encode(format!("{}:{}", config.user, config.password));
    let request_string = serde_json::to_string(&request)?;

    let mut headers = String::new();
    headers.push_str(&format!("POST / HTTP/1.1\r\n"));
    headers.push_str(&format!("Host: {}:{}\r\n", config.host, config.port));
    headers.push_str("Content-Type: application/json\r\n");
    headers.push_str(&format!("Authorization: Basic {}\r\n", auth));
    headers.push_str(&format!("Content-Length: {}\r\n", request_string.len()));
    headers.push_str("\r\n");

    stream.write_all(headers.as_bytes())?;
    stream.write_all(request_string.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    let response_json: Value = serde_json::from_str(&response)?;

    if let Some(error) = response_json.get("error") {
        if !error.is_null() {
            return Err(RpcError::Rpc(error.to_string()));
        }
    }

    Ok(response_json["result"].clone())
}

/// Main function for the bitrpc CLI
fn main() -> Result<(), RpcError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: bitrpc <command> [params...]");
        return Ok(());
    }

    let config = RpcConfig {
        host: "127.0.0.1".to_string(),
        port: 8332, // Default RPC port for BitcoinZ
        user: "rpcuser".to_string(),
        password: "rpcpassword".to_string(),
    };

    let method = &args[1];
    let params: Vec<Value> = args[2..]
        .iter()
        .map(|arg| serde_json::Value::String(arg.to_string()))
        .collect();

    match send_rpc_command(&config, method, &params) {
        Ok(result) => println!("{}", serde_json::to_string_pretty(&result)?),
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}
