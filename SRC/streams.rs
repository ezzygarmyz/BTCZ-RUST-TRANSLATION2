use std::io::{self, Read, Write, Seek, SeekFrom};
use thiserror::Error;

/// Custom error for stream operations
#[derive(Debug, Error)]
pub enum StreamError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Stream is out of bounds")]
    OutOfBounds,
}

/// A dynamic byte buffer for serialized data
pub struct DataStream {
    buffer: Vec<u8>,
    position: usize,
}

impl DataStream {
    /// Creates a new empty DataStream
    pub fn new() -> Self {
        DataStream {
            buffer: Vec::new(),
            position: 0,
        }
    }

    /// Creates a DataStream with an initial buffer
    pub fn from_buffer(buffer: Vec<u8>) -> Self {
        DataStream { buffer, position: 0 }
    }

    /// Writes raw bytes to the stream
    pub fn write(&mut self, data: &[u8]) -> Result<(), StreamError> {
        self.buffer.extend_from_slice(data);
        Ok(())
    }

    /// Reads raw bytes from the stream
    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, StreamError> {
        if self.position + size > self.buffer.len() {
            return Err(StreamError::OutOfBounds);
        }
        let data = self.buffer[self.position..self.position + size].to_vec();
        self.position += size;
        Ok(data)
    }

    /// Reads a single byte from the stream
    pub fn read_byte(&mut self) -> Result<u8, StreamError> {
        if self.position >= self.buffer.len() {
            return Err(StreamError::OutOfBounds);
        }
        let byte = self.buffer[self.position];
        self.position += 1;
        Ok(byte)
    }

    /// Seeks to a specific position in the stream
    pub fn seek(&mut self, position: usize) -> Result<(), StreamError> {
        if position > self.buffer.len() {
            return Err(StreamError::OutOfBounds);
        }
        self.position = position;
        Ok(())
    }

    /// Returns the current position in the stream
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns the size of the stream
    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    /// Clears the stream
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.position = 0;
    }
}

/// A buffered file reader and writer for serialized data
pub struct BufferedFile<R: Read + Seek> {
    reader: R,
    buffer: Vec<u8>,
    position: usize,
    limit: usize,
}

impl<R: Read + Seek> BufferedFile<R> {
    /// Creates a new BufferedFile with a buffer limit
    pub fn new(reader: R, limit: usize) -> Self {
        BufferedFile {
            reader,
            buffer: Vec::new(),
            position: 0,
            limit,
        }
    }

    /// Reads data into the buffer
    pub fn fill_buffer(&mut self) -> Result<(), StreamError> {
        let mut temp_buffer = vec![0; self.limit];
        let bytes_read = self.reader.read(&mut temp_buffer)?;
        self.buffer = temp_buffer[..bytes_read].to_vec();
        self.position = 0;
        Ok(())
    }

    /// Reads a specific number of bytes from the buffer
    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, StreamError> {
        if self.position + size > self.buffer.len() {
            return Err(StreamError::OutOfBounds);
        }
        let data = self.buffer[self.position..self.position + size].to_vec();
        self.position += size;
        Ok(data)
    }

    /// Resets the file position and buffer
    pub fn reset(&mut self) -> Result<(), StreamError> {
        self.reader.seek(SeekFrom::Start(0))?;
        self.fill_buffer()
    }
}
