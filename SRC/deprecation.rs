use std::error::Error;
use log::warn;

/// Defines the block height at which this version is deprecated.
const DEPRECATION_HEIGHT: u32 = 1_000_000; // Example deprecation height

/// Number of blocks before deprecation to start warning users.
const WARNING_PERIOD: u32 = 10_000;

/// Checks if the node is running a deprecated version and enforces deprecation.
pub fn enforce_node_deprecation(current_height: u32) -> Result<(), Box<dyn Error>> {
    if current_height >= DEPRECATION_HEIGHT {
        return Err("This version has been deprecated. Please update to the latest version.".into());
    }
    Ok(())
}

/// Logs a warning if the version is nearing deprecation.
pub fn warn_if_deprecated(current_height: u32) {
    if current_height >= DEPRECATION_HEIGHT - WARNING_PERIOD {
        warn!("Warning: This version is nearing deprecation.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_enforce_node_deprecation() {
        assert!(enforce_node_deprecation(999_999).is_ok());
        assert!(enforce_node_deprecation(1_000_000).is_err());
    }

    #[test]
    fn test_warn_if_deprecated() {
        let mut warnings = vec![];
        let warning_logger = |msg: &str| warnings.push(msg.to_string());

        warn_if_deprecated(990_000); // No warning
        assert!(warnings.is_empty());

        warn_if_deprecated(990_001); // Warning
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0], "Warning: This version is nearing deprecation.");
    }
}
