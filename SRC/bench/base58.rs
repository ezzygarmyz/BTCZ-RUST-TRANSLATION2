use bitcoin::util::base58;

pub fn benchmark_base58_encoding() {
    let data = b"Benchmarking Base58 encoding";
    let encoded = base58::encode(data);
    println!("Base58 Encoded: {}", encoded);
}

pub fn benchmark_base58_decoding() {
    let encoded = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let decoded = base58::decode(encoded).expect("Failed to decode Base58");
    println!("Base58 Decoded: {:?}", decoded);
}
