# Installation Guide for BitcoinZ Rust Implementation

This guide provides step-by-step instructions for installing and running the BitcoinZ Rust implementation. Follow the procedures below to set up your environment and ensure proper functionality.

---

## Prerequisites

Ensure your system meets the following requirements:

### System Requirements
- **Operating System**: Linux, macOS, or Windows
- **Memory**: Minimum 4 GB of RAM (8 GB recommended)
- **Storage**: Minimum 100 GB of free disk space
- **Processor**: x86_64 architecture with at least 2 cores

### Software Requirements
- **Rust Compiler**: Install the Rust toolchain (stable version)
- **Cargo**: Rust’s package manager and build system
- **Git**: Version control system for cloning the repository

---

## Installation Steps

### 1. Install Rust Toolchain
Install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup update
Verify the installation:

rustc --version
cargo --version
2. Clone the Repository
Clone the BitcoinZ Rust implementation repository:

git clone https://github.com/jeelybeely/BTCZ-RUST-TRANSLATION2.git
cd BTCZ-RUST-TRANSLATION2
3. Build the Project
Build the project using cargo:

cargo build --release
The compiled binaries will be located in the target/release directory.

4. Configure the Node
Create a configuration file for the node:

mkdir -p ~/.bitcoinz
nano ~/.bitcoinz/bitcoinz.conf
Example configuration:

rpcuser=your_rpc_user
rpcpassword=your_rpc_password
rpcport=8332
datadir=/path/to/data
testnet=0
txindex=1
5. Start the Node
Run the BitcoinZ node with the following command:

./target/release/bitcoind --config ~/.bitcoinz/bitcoinz.conf
Verify that the node is running and syncing with the blockchain.

6. Interact with the Node
Use the bitcoinz-cli tool to interact with the running node. Example commands:

Check blockchain info:

./target/release/bitcoinz-cli getblockchaininfo
View network info:

./target/release/bitcoinz-cli getnetworkinfo
Additional Notes
Testnet Configuration
To use the testnet, modify the bitcoinz.conf file:

testnet=1
Restart the node for the changes to take effect.

Docker Deployment
For Docker users, you can use the provided Dockerfile to build and run the node in a container:

docker build -t bitcoinz-node .
docker run -d -v ~/.bitcoinz:/data -p 8332:8332 bitcoinz-node
Troubleshooting
Common Issues
Build Errors: Ensure all dependencies are installed and the Rust toolchain is up to date.
Node Sync Issues: Verify your internet connection and check the bitcoinz.conf settings.
Logs and Debugging
View logs in the data directory:

tail -f /path/to/data/debug.log
Uninstallation
To remove the BitcoinZ Rust implementation from your system:

Delete the binaries and source code:

rm -rf BTCZ-RUST-TRANSLATION2
Remove configuration and blockchain data:

rm -rf ~/.bitcoinz
Support
For further assistance, visit the BitcoinZ Community Forums or join the BitcoinZ Discord.