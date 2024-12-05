use serde_json::Value;
use std::fs::File;
use std::io::{self, Write};
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

    let start_height = config["start_end"]["start_height"].as_integer().unwrap();
    let end_height = config["start_end"]["end_height"].as_integer().unwrap();

    // Create HTTP client
    let client = Client::new();

    // Open output file
    let mut hashlist_file = File::create(config["hashlist"].as_str().unwrap())?;

    for height in start_height..=end_height {
        let request_body = json!({
            "jsonrpc": "1.0",
            "id": "linearize",
            "method": "getblockhash",
            "params": [height]
        });

        let response: Value = client
            .post(&rpc_url)
            .json(&request_body)
            .send()
            .expect("Failed to send RPC request")
            .json()
            .expect("Invalid JSON response");

        let hash = response["result"].as_str().unwrap();
        writeln!(hashlist_file, "{}", hash)?;
    }

    println!("Hash list successfully generated!");
    Ok(())
}
