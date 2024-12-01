mod logging;
mod network;
mod consensus;
mod wallet;
mod mempool;

use clap::{App, Arg};
use log::{info, error};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Application configuration
struct AppConfig {
    data_dir: String,
    log_file: String,
    network_port: u16,
}

/// Initializes the application configuration
fn init_config(matches: clap::ArgMatches) -> AppConfig {
    let data_dir = matches
        .value_of("data_dir")
        .unwrap_or("./data")
        .to_string();
    let log_file = matches
        .value_of("log_file")
        .unwrap_or("bitcoinz.log")
        .to_string();
    let network_port = matches
        .value_of("port")
        .unwrap_or("8333")
        .parse::<u16>()
        .unwrap_or(8333);

    AppConfig {
        data_dir,
        log_file,
        network_port,
    }
}

/// Initializes logging
fn init_logging(log_file: &str) {
    let config = logging::LoggerConfig {
        log_to_console: true,
        log_to_file: true,
        file_path: Some(log_file.to_string()),
        verbosity: log::LevelFilter::Info,
    };

    if let Err(e) = logging::init_logger(config) {
        eprintln!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }
}

/// Starts the networking service
async fn start_network_service(port: u16) -> Result<(), String> {
    let network = Arc::new(network::Network::new());
    info!("Starting network listener on port {}", port);
    network.start_listener(&format!("0.0.0.0:{}", port)).await
}

/// Starts the application
fn run_app(config: AppConfig) -> Result<(), String> {
    // Initialize logging
    init_logging(&config.log_file);

    // Start the runtime
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    // Start services
    runtime.block_on(async {
        info!("BitcoinZ Node Starting...");
        info!("Data directory: {}", config.data_dir);

        if let Err(e) = start_network_service(config.network_port).await {
            error!("Network service failed: {}", e);
            return Err(e);
        }

        info!("BitcoinZ Node Running.");
        Ok(())
    })
}

/// Main entry point
fn main() {
    // Parse command-line arguments
    let matches = App::new("BitcoinZ Node")
        .version("1.0")
        .about("BitcoinZ Full Node")
        .arg(
            Arg::new("data_dir")
                .short('d')
                .long("data-dir")
                .takes_value(true)
                .help("Specify the data directory"),
        )
        .arg(
            Arg::new("log_file")
                .short('l')
                .long("log-file")
                .takes_value(true)
                .help("Specify the log file path"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .takes_value(true)
                .help("Specify the network port"),
        )
        .get_matches();

    // Initialize application configuration
    let config = init_config(matches);

    // Run the application
    if let Err(e) = run_app(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
