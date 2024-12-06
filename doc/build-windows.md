Building BitcoinZ Rust Node on Windows
System Requirements
To build and run the BitcoinZ node on Windows, ensure your system meets the following requirements:

Operating System: Windows 10 or later (64-bit)
Processor: x64 architecture
Memory: At least 8 GB RAM
Storage: At least 10 GB of free disk space
Software:
Rust programming language (latest stable version)
Git for version control
Visual Studio (with C++ Build Tools installed)
PowerShell (pre-installed on Windows 10 or later)
1. Install Required Tools
Rust
Download and install the Rust programming language via Rustup:

Invoke-WebRequest -Uri https://sh.rustup.rs -OutFile rustup-init.exe
.\rustup-init.exe
Follow the installation prompts and ensure that cargo, rustc, and rustup are added to your system PATH.

Git
Install Git from git-scm.com and verify installation:

git --version
Visual Studio
Download and install Visual Studio Community Edition from visualstudio.com. During installation:

Select Desktop development with C++.
Include the C++ build tools workload.
OpenSSL
Install OpenSSL using the vcpkg package manager:

git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg.exe install openssl:x64-windows
Add the vcpkg installed\x64-windows\bin directory to your system PATH.

2. Clone the Repository
Clone the BitcoinZ Rust translation repository:

git clone https://github.com/jeelybeely/BTCZ-RUST-TRANSLATION2.git
cd BTCZ-RUST-TRANSLATION2
3. Build the Project
Run the following command to compile the project in release mode:

cargo build --release
This will create an optimized binary in the target\release directory.

4. Run the BitcoinZ Node
Start the BitcoinZ node with the following command:

.\target\release\bitcoind.exe --config path\to\bitcoinz.conf
5. Running Tests
To verify the build, execute the test suite:

cargo test
This will run unit and integration tests to ensure functionality.

6. Optional: Running in a Docker Environment
If you prefer containerized execution, you can build and run the node using Docker:

Install Docker Desktop for Windows from docker.com.
Build the Docker image:
docker build -t bitcoinz-node .
Run the container:
docker run -d --name bitcoinz-node -p 8333:8333 -p 8332:8332 bitcoinz-node
7. Known Issues
Ensure your Windows system PATH includes all required tools (cargo, rustc, OpenSSL binaries, etc.).
If the build fails, check that the correct Visual Studio version and workloads are installed.
8. Additional Resources
Rust Documentation
BitcoinZ GitHub Repository
BitcoinZ Official Website
