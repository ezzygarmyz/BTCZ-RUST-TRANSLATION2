use std::io::{self, Read};

/// Reads a 32-bit unsigned integer from a stream.
pub fn read_u32<R: Read>(reader: &mut R) -> Result<u32, String> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer).map_err(|e| e.to_string())?;
    Ok(u32::from_le_bytes(buffer))
}

/// Reads a 64-bit unsigned integer from a stream.
pub fn read_u64<R: Read>(reader: &mut R) -> Result<u64, String> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer).map_err(|e| e.to_string())?;
    Ok(u64::from_le_bytes(buffer))
}

/// Reads a variable-length integer (varint) from a stream.
pub fn read_varint<R: Read>(reader: &mut R) -> Result<u64, String> {
    let mut buffer = [0; 1];
    reader.read_exact(&mut buffer).map_err(|e| e.to_string())?;
    let prefix = buffer[0];

    match prefix {
        0xFD => read_u16(reader).map(|v| v as u64),
        0xFE => read_u32(reader).map(|v| v as u64),
        0xFF => read_u64(reader),
        _ => Ok(prefix as u64),
    }
}

/// Reads a 16-bit unsigned integer from a stream.
fn read_u16<R: Read>(reader: &mut R) -> Result<u16, String> {
    let mut buffer = [0; 2];
    reader.read_exact(&mut buffer).map_err(|e| e.to_string())?;
    Ok(u16::from_le_bytes(buffer))
}
