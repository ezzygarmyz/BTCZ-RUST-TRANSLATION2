use clap::{App, Arg};
use reqwest::Client;
use serde_json::Value;
use std::process::exit;

/// Sends an RPC request to the BitcoinZ daemon.
async fn call_rpc(command: &str, params: Vec<&str>) -> Result<Value, Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:8232"; // Default RPC URL for BitcoinZ
    let username = "rpcuser";          // Replace with your RPC username
    let password = "rpcpassword";      // Replace with your RPC password

    let client = Client::new();
    let payload = serde_json::json!({
        "jsonrpc": "1.0",
        "id": "bitcoinz-cli",
        "method": command,
        "params": params,
    });

    let response = client
        .post(url)
        .basic_auth(username, Some(password))
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() {
    let matches = App::new("bitcoinz-cli")
        .version("0.1.0")
        .author("BitcoinZ Developers")
        .about("Command Line Interface for BitcoinZ")
        .arg(
            Arg::new("command")
                .about("The RPC command to execute")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("params")
                .about("Parameters for the RPC command")
                .multiple_occurrences(true),
        )
        .get_matches();

    let command = matches.value_of("command").unwrap();
    let params = matches
        .values_of("params")
        .unwrap_or_default()
        .collect::<Vec<&str>>();

    match call_rpc(command, params).await {
        Ok(response) => {
            println!("{}", serde_json::to_string_pretty(&response).unwrap());
        }
        Err(e) => {
            eprintln!("RPC Error: {}", e);
            exit(1);
        }
    }
}
