use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Represents an address entry in AddrMan.
#[derive(Debug)]
pub struct AddrInfo {
    pub ip: String,
    pub port: u16,
    pub timestamp: u64,
}

/// Represents the Address Manager (`AddrMan`) structure.
#[derive(Debug)]
pub struct AddrMan {
    pub addresses: Vec<AddrInfo>,
}

impl AddrMan {
    /// Creates a new, empty AddrMan instance.
    pub fn new() -> Self {
        AddrMan {
            addresses: Vec::new(),
        }
    }

    /// Loads serialized data from a file.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Deserialize data (skipping the first `D` bytes if needed)
        let deserialized = AddrMan::deserialize(&buffer)?;
        Ok(deserialized)
    }

    /// Deserializes raw binary data into an AddrMan structure.
    pub fn deserialize(data: &[u8]) -> io::Result<Self> {
        let mut addresses = Vec::new();
        let mut cursor = 0;

        // Example decoding logic (this should match the actual binary format)
        while cursor < data.len() {
            if data.len() - cursor < 12 {
                break; // Avoid reading beyond the buffer
            }

            // Decode an IP address (4 bytes), port (2 bytes), and timestamp (8 bytes)
            let ip_bytes = &data[cursor..cursor + 4];
            let port_bytes = &data[cursor + 4..cursor + 6];
            let timestamp_bytes = &data[cursor + 6..cursor + 14];

            let ip = format!(
                "{}.{}.{}.{}",
                ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]
            );
            let port = u16::from_be_bytes([port_bytes[0], port_bytes[1]]);
            let timestamp = u64::from_le_bytes([
                timestamp_bytes[0], timestamp_bytes[1], timestamp_bytes[2], timestamp_bytes[3],
                timestamp_bytes[4], timestamp_bytes[5], timestamp_bytes[6], timestamp_bytes[7],
            ]);

            addresses.push(AddrInfo { ip, port, timestamp });
            cursor += 14; // Move cursor to the next record
        }

        Ok(AddrMan { addresses })
    }

    /// Displays all loaded addresses.
    pub fn display_addresses(&self) {
        for addr in &self.addresses {
            println!("{:?}", addr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_addrman() {
        let data = vec![
            192, 168, 1, 1, 0x1F, 0x90, 0x60, 0xEA, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let addrman = AddrMan::deserialize(&data).unwrap();
        assert_eq!(addrman.addresses.len(), 1);
        assert_eq!(addrman.addresses[0].ip, "192.168.1.1");
        assert_eq!(addrman.addresses[0].port, 8080);
    }
}
