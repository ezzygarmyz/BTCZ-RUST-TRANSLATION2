use std::io::{self, Read, Write};
use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Custom error for serialization/deserialization
#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Invalid data format")]
    InvalidData,
}

/// Trait for serializing data to a writable stream
pub trait Serializable {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerializationError>;
}

/// Trait for deserializing data from a readable stream
pub trait Deserializable: Sized {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, SerializationError>;
}

/// Compact size representation (used for variable-length integers in BitcoinZ)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct CompactSize(pub u64);

impl CompactSize {
    /// Encodes the compact size into a writer
    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerializationError> {
        match self.0 {
            0..=0xFC => writer.write_all(&[self.0 as u8])?,
            0xFD..=0xFFFF => {
                writer.write_all(&[0xFD])?;
                writer.write_all(&(self.0 as u16).to_le_bytes())?;
            }
            0x10000..=0xFFFFFFFF => {
                writer.write_all(&[0xFE])?;
                writer.write_all(&(self.0 as u32).to_le_bytes())?;
            }
            _ => {
                writer.write_all(&[0xFF])?;
                writer.write_all(&(self.0 as u64).to_le_bytes())?;
            }
        }
        Ok(())
    }

    /// Decodes the compact size from a reader
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, SerializationError> {
        let mut prefix = [0u8; 1];
        reader.read_exact(&mut prefix)?;

        let value = match prefix[0] {
            0xFF => {
                let mut buf = [0u8; 8];
                reader.read_exact(&mut buf)?;
                u64::from_le_bytes(buf)
            }
            0xFE => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                u32::from_le_bytes(buf) as u64
            }
            0xFD => {
                let mut buf = [0u8; 2];
                reader.read_exact(&mut buf)?;
                u16::from_le_bytes(buf) as u64
            }
            x => x as u64,
        };

        Ok(CompactSize(value))
    }
}

/// A helper struct for serializing and deserializing BitcoinZ data
#[derive(Debug)]
pub struct SerializeHelper;

impl SerializeHelper {
    /// Writes a vector of bytes
    pub fn write_bytes<W: Write>(writer: &mut W, data: &[u8]) -> Result<(), SerializationError> {
        CompactSize(data.len() as u64).serialize(writer)?;
        writer.write_all(data)?;
        Ok(())
    }

    /// Reads a vector of bytes
    pub fn read_bytes<R: Read>(reader: &mut R) -> Result<Vec<u8>, SerializationError> {
        let size = CompactSize::deserialize(reader)?.0 as usize;
        let mut buffer = vec![0; size];
        reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}
