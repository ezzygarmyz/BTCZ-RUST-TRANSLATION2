use std::io::{self, Read, Write};
use crate::utils::hash::Hash256;
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Custom error type for protocol operations
#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Invalid message type")]
    InvalidMessageType,
}

/// Protocol message types
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Version,
    Verack,
    Ping,
    Pong,
    Addr,
    Inv,
    GetData,
    Block,
    Tx,
    Unknown,
}

impl MessageType {
    /// Parses a message type from a string
    pub fn from_str(message: &str) -> Self {
        match message {
            "version" => MessageType::Version,
            "verack" => MessageType::Verack,
            "ping" => MessageType::Ping,
            "pong" => MessageType::Pong,
            "addr" => MessageType::Addr,
            "inv" => MessageType::Inv,
            "getdata" => MessageType::GetData,
            "block" => MessageType::Block,
            "tx" => MessageType::Tx,
            _ => MessageType::Unknown,
        }
    }

    /// Converts a message type to a string
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::Version => "version",
            MessageType::Verack => "verack",
            MessageType::Ping => "ping",
            MessageType::Pong => "pong",
            MessageType::Addr => "addr",
            MessageType::Inv => "inv",
            MessageType::GetData => "getdata",
            MessageType::Block => "block",
            MessageType::Tx => "tx",
            MessageType::Unknown => "unknown",
        }
    }
}

/// Protocol message header
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageHeader {
    magic: u32,
    command: String,
    length: u32,
    checksum: Hash256,
}

impl MessageHeader {
    /// Serializes the message header
    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), ProtocolError> {
        writer.write_all(&self.magic.to_le_bytes())?;
        let mut command_bytes = [0u8; 12];
        command_bytes[..self.command.len()].copy_from_slice(self.command.as_bytes());
        writer.write_all(&command_bytes)?;
        writer.write_all(&self.length.to_le_bytes())?;
        writer.write_all(&self.checksum.0)?;
        Ok(())
    }

    /// Deserializes a message header
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, ProtocolError> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        let magic = u32::from_le_bytes(magic);

        let mut command_bytes = [0u8; 12];
        reader.read_exact(&mut command_bytes)?;
        let command = String::from_utf8_lossy(&command_bytes).trim_end_matches('\0').to_string();

        let mut length = [0u8; 4];
        reader.read_exact(&mut length)?;
        let length = u32::from_le_bytes(length);

        let mut checksum = [0u8; 32];
        reader.read_exact(&mut checksum)?;

        Ok(MessageHeader {
            magic,
            command,
            length,
            checksum: Hash256(checksum),
        })
    }
}

/// Protocol message
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    header: MessageHeader,
    payload: Vec<u8>,
}

impl Message {
    /// Serializes the message
    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), ProtocolError> {
        self.header.serialize(writer)?;
        writer.write_all(&self.payload)?;
        Ok(())
    }

    /// Deserializes a message
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, ProtocolError> {
        let header = MessageHeader::deserialize(reader)?;
        let mut payload = vec![0; header.length as usize];
        reader.read_exact(&mut payload)?;
        Ok(Message { header, payload })
    }
}
