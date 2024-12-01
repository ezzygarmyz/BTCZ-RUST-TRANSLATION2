use sled::{Db, IVec};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;

/// Represents a network address.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkAddress {
    pub ip: String,
    pub port: u16,
    pub last_seen: u64, // Timestamp of the last activity
}

/// Database wrapper for storing and retrieving network addresses.
pub struct AddressDatabase {
    db: Db,
}

impl AddressDatabase {
    /// Opens the address database.
    pub fn open(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(AddressDatabase { db })
    }

    /// Reads all addresses from the database.
    pub fn read_all(&self) -> Result<Vec<NetworkAddress>, SerdeError> {
        self.db
            .iter()
            .values()
            .filter_map(|res| res.ok()) // Filter valid entries
            .map(|ivec| serde_json::from_slice(&ivec)) // Deserialize
            .collect()
    }

    /// Writes a single address to the database.
    pub fn write(&self, address: &NetworkAddress) -> Result<(), sled::Error> {
        let key = format!("{}:{}", address.ip, address.port);
        let value = serde_json::to_vec(address)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    /// Deletes an address from the database.
    pub fn delete(&self, ip: &str, port: u16) -> Result<(), sled::Error> {
        let key = format!("{}:{}", ip, port);
        self.db.remove(key)?;
        Ok(())
    }

    /// Flushes changes to disk.
    pub fn flush(&self) -> Result<(), sled::Error> {
        self.db.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_database() {
        let db = AddressDatabase::open("test_addrdb").unwrap();

        // Create a test address
        let addr = NetworkAddress {
            ip: "127.0.0.1".to_string(),
            port: 8333,
            last_seen: 1633017600,
        };

        // Write the address to the database
        db.write(&addr).unwrap();

        // Read all addresses
        let addresses = db.read_all().unwrap();
        assert!(addresses.contains(&addr));

        // Delete the address
        db.delete(&addr.ip, addr.port).unwrap();

        // Ensure the address is deleted
        let addresses = db.read_all().unwrap();
        assert!(!addresses.contains(&addr));

        // Cleanup
        db.flush().unwrap();
    }
}
