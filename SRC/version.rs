/// Current protocol version
pub const PROTOCOL_VERSION: i32 = 170013;

/// Minimum protocol version required for compatibility
pub const MIN_PROTOCOL_VERSION: i32 = 170012;

/// Version when `nTime` field was added to CAddress
pub const CADDR_TIME_VERSION: i32 = 31402;

/// Version when BIP 0031 (pong message) was introduced
pub const BIP0031_VERSION: i32 = 60000;

/// Version when `filter*` messages are disabled without NODE_BLOOM
pub const NO_BLOOM_VERSION: i32 = 70011;

/// Version when `sendheaders` was introduced
pub const SENDHEADERS_VERSION: i32 = 70012;

/// Software version string
pub const CLIENT_VERSION_STR: &str = "BitcoinZ Core v1.3.0";

/// Initializes versioning information
pub fn get_version_info() -> VersionInfo {
    VersionInfo {
        protocol_version: PROTOCOL_VERSION,
        min_protocol_version: MIN_PROTOCOL_VERSION,
        client_version_str: CLIENT_VERSION_STR.to_string(),
    }
}

/// Struct to encapsulate version information
#[derive(Debug)]
pub struct VersionInfo {
    pub protocol_version: i32,
    pub min_protocol_version: i32,
    pub client_version_str: String,
}
