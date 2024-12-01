use sled::{Db, IVec};

/// Wrapper for the Sled database.
pub struct BitcoinZDatabase {
    db: Db,
}

impl BitcoinZDatabase {
    /// Opens a database at the specified path.
    pub fn open(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(BitcoinZDatabase { db })
    }

    /// Inserts a key-value pair into the database.
    pub fn insert(&self, key: &[u8], value: &[u8]) -> Result<Option<IVec>, sled::Error> {
        self.db.insert(key, value)
    }

    /// Retrieves a value by key.
    pub fn get(&self, key: &[u8]) -> Result<Option<IVec>, sled::Error> {
        self.db.get(key)
    }

    /// Deletes a key-value pair by key.
    pub fn remove(&self, key: &[u8]) -> Result<Option<IVec>, sled::Error> {
        self.db.remove(key)
    }

    /// Flushes all changes to disk.
    pub fn flush(&self) -> Result<(), sled::Error> {
        self.db.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_operations() {
        let db = BitcoinZDatabase::open("test_db").unwrap();

        // Insert a key-value pair
        db.insert(b"key1", b"value1").unwrap();

        // Retrieve the value
        let value = db.get(b"key1").unwrap().unwrap();
        assert_eq!(value, b"value1");

        // Delete the key-value pair
        db.remove(b"key1").unwrap();

        // Ensure the key is removed
        let value = db.get(b"key1").unwrap();
        assert!(value.is_none());

        // Cleanup
        db.flush().unwrap();
    }
}
