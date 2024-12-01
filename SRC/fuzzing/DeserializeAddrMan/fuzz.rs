use crate::addrman::{AddrMan, deserialize_addrman};
use std::io::{Cursor, ErrorKind};
use fuzzing::FuzzedDataProvider;

pub fn initialize_addrman() {
    println!("Initializing AddrMan for fuzz testing...");
}

/// Fuzz test entry point for deserializing AddrMan objects
pub fn fuzz_deserialize_addrman(data: &[u8]) {
    let fuzzed_data_provider = FuzzedDataProvider::new(data);

    // Create AddrMan object
    let mut addrman = AddrMan::new();

    // Attempt deserialization
    if let Err(e) = deserialize_addrman(&mut addrman, fuzzed_data_provider.consume_random_bytes(1024)) {
        match e.kind() {
            ErrorKind::InvalidData => println!("Deserialization failed due to invalid data."),
            _ => println!("Unexpected deserialization error: {:?}", e),
        }
    }
}
