use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use std::time::Duration;

const DEFAULT_TOR_CONTROL_PORT: u16 = 9051;
const DEFAULT_TOR_COOKIE_FILE: &str = "/var/lib/tor/control_auth_cookie";

/// Custom errors for TorControl operations
#[derive(Debug, Error)]
pub enum TorControlError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid Tor response")]
    InvalidResponse,
}

/// Tor Control client for managing Tor integration
pub struct TorControl {
    address: String,
    auth_cookie: Option<Vec<u8>>,
}

impl TorControl {
    /// Creates a new TorControl instance
    pub fn new(address: String) -> Self {
        TorControl {
            address,
            auth_cookie: None,
        }
    }

    /// Connects to the Tor Control port
    pub async fn connect(&self) -> Result<TcpStream, TorControlError> {
        let stream = TcpStream::connect(&self.address).await?;
        Ok(stream)
    }

    /// Authenticates with the Tor daemon
    pub async fn authenticate(
        &self,
        stream: &mut TcpStream,
        cookie_file: Option<&str>,
    ) -> Result<(), TorControlError> {
        let cookie = match cookie_file {
            Some(path) => std::fs::read(path)?,
            None => std::fs::read(DEFAULT_TOR_COOKIE_FILE)?,
        };

        self.send_command(stream, &format!("AUTHCOOKIE {}", hex::encode(cookie)))
            .await?;
        let response = self.receive_response(stream).await?;

        if !response.starts_with("250 OK") {
            return Err(TorControlError::InvalidResponse);
        }

        Ok(())
    }

    /// Creates an onion service
    pub async fn create_onion_service(
        &self,
        stream: &mut TcpStream,
        port: u16,
    ) -> Result<String, TorControlError> {
        self.send_command(stream, &format!("ADD_ONION NEW:BEST Port={},127.0.0.1:{}", port, port))
            .await?;
        let response = self.receive_response(stream).await?;

        if let Some(line) = response.lines().find(|line| line.starts_with("250-ServiceID=")) {
            let service_id = line.split('=').nth(1).unwrap_or_default();
            Ok(service_id.to_string())
        } else {
            Err(TorControlError::InvalidResponse)
        }
    }

    /// Sends a command to the Tor daemon
    async fn send_command(
        &self,
        stream: &mut TcpStream,
        command: &str,
    ) -> Result<(), TorControlError> {
        let command_with_newline = format!("{}\r\n", command);
        stream.write_all(command_with_newline.as_bytes()).await?;
        Ok(())
    }

    /// Receives a response from the Tor daemon
    async fn receive_response(&self, stream: &mut TcpStream) -> Result<String, TorControlError> {
        let mut buffer = vec![0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        Ok(response)
    }
}
