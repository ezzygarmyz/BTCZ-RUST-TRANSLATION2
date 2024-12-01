use sled::{Db, IVec};
use std::path::Path;

/// Represents a database wrapper for key-value storage.
pub struct DBWrapper {
    db: Db,
}

impl DBWrapper {
    /// Creates a new database wrapper.
    pub fn new(path: &Path) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(DBWrapper { db })
    }

    /// Reads a value from the database by key.
    pub fn read(&self, key: &str) -> Option<String> {
        self.db
            .get(key)
            .ok()
            .flatten()
            .map(|ivec| String::from_utf8_lossy(&ivec).to_string())
    }

    /// Writes a key-value pair to the database.
    pub fn write(&self, key: &str, value: &str) -> Result<(), sled::Error> {
        self.db.insert(key, value.as_bytes())?;
        Ok(())
    }

    /// Deletes a key-value pair from the database.
    pub fn erase(&self, key: &str) -> Result<(), sled::Error> {
        self.db.remove(key)?;
        Ok(())
    }

    /// Flushes the database to disk.
    pub fn flush(&self) -> Result<(), sled::Error> {
        self.db.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_db_operations() {
        // Create a temporary directory for testing
        let mut temp_dir = PathBuf::from(env::temp_dir());
        temp_dir.push("dbwrapper_test");

        let db = DBWrapper::new(&temp_dir).unwrap();

        // Test writing
        db.write("key1", "value1").unwrap();
        db.write("key2", "value2").unwrap();

        // Test reading
        assert_eq!(db.read("key1").unwrap(), "value1");
        assert_eq!(db.read("key2").unwrap(), "value2");
        assert!(db.read("key3").is_none());

        // Test erasing
        db.erase("key1").unwrap();
        assert!(db.read("key1").is_none());

        // Flush the database
        db.flush().unwrap();

        // Clean up
        std::fs::remove_dir_all(temp_dir).unwrap();
    }
}
