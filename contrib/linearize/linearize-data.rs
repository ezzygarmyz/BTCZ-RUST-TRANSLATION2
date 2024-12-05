use serde_json::Value;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use reqwest::blocking::Client;

fn main() -> io::Result<()> {
    // Load configuration
    let config = std::fs::read_to_string("example-linearize.toml")
        .expect("Failed to load configuration file");
    let config: toml::Value = toml::from_str(&config).expect("Invalid TOML format");

    let rpc_url = format!(
        "http://{}:{}@127.0.0.1:{}",
        config["rpc"]["user"].as_str().unwrap(),
        config["rpc"]["password"].as_str().unwrap(),
        config["rpc"]["port"].as_integer().unwrap()
    );

    let hashlist_path = config["hashlist"].as_str().unwrap();
    let output_file_path = config["output_file"].as_str().unwrap();

    // Read hash list
    let hashlist = fs::File::open(hashlist_path)?;
    let reader = io::BufReader::new(hashlist);

    // Open output file
    let mut output_file = File::create(output_file_path)?;

    // Create HTTP client
    let client = Client::new();

    for line in reader.lines() {
        let hash = line.expect("Failed to read hash");
        let request_body = json!({
            "jsonrpc": "1.0",
            "id": "linearize",
            "method": "getblock",
            "params": [hash]
        });

        let response: Value = client
            .post(&rpc_url)
            .json(&request_body)
            .send()
            .expect("Failed to send RPC request")
            .json()
            .expect("Invalid JSON response");

        let block_data = response["result"].as_str().unwrap();
        output_file.write_all(block_data.as_bytes())?;
    }

    println!("Linearized blockchain data successfully generated!");
    Ok(())
}
