use std::fs;
use std::path::{Path, PathBuf};
use std::io;

/// Checks if a path exists.
pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Creates directories for a given path, including intermediate directories.
pub fn create_directories(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Retrieves the size of a file in bytes.
pub fn file_size(path: &Path) -> io::Result<u64> {
    fs::metadata(path).map(|metadata| metadata.len())
}

/// Joins two paths into a normalized PathBuf.
pub fn join_paths(base: &Path, relative: &Path) -> PathBuf {
    base.join(relative)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;

    #[test]
    fn test_path_operations() {
        // Create a temporary directory for testing
        let temp_dir = env::temp_dir().join("fs_test");
        create_directories(&temp_dir).unwrap();

        // Test path existence
        assert!(path_exists(&temp_dir));

        // Create a file in the directory
        let file_path = temp_dir.join("test_file.txt");
        File::create(&file_path).unwrap();

        // Test file size
        assert_eq!(file_size(&file_path).unwrap(), 0);

        // Test path joining
        let joined_path = join_paths(&temp_dir, Path::new("subdir"));
        assert_eq!(joined_path, temp_dir.join("subdir"));

        // Clean up
        fs::remove_file(&file_path).unwrap();
        fs::remove_dir(&temp_dir).unwrap();
    }
}
