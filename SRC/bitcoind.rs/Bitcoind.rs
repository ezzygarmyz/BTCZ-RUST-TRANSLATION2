use bitcoinz::init::{app_init, app_shutdown};
use bitcoinz::logging::setup_logger;
use bitcoinz::net::start_network;
use bitcoinz::rpc::start_rpc_server;
use std::process;

#[tokio::main]
async fn main() {
    // Initialize logging
    if let Err(e) = setup_logger() {
        eprintln!("Error: Failed to initialize logger: {}", e);
        process::exit(1);
    }

    // Initialize application components
    if let Err(e) = app_init().await {
        eprintln!("Error: Initialization failed: {}", e);
        process::exit(1);
    }

    // Start network services
    if let Err(e) = start_network().await {
        eprintln!("Error: Failed to start network services: {}", e);
        process::exit(1);
    }

    // Start RPC server
    if let Err(e) = start_rpc_server().await {
        eprintln!("Error: Failed to start RPC server: {}", e);
        process::exit(1);
    }

    // Main loop
    tokio::spawn(async {
        loop {
            // Process network events and transactions
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    // Handle shutdown gracefully
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");
    app_shutdown().await;
}
