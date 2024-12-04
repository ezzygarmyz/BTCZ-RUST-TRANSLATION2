use std::process::{Command, Stdio};
use std::fs::{self, File};
use std::path::Path;

/// Directory containing binaries to check
const BINARIES_DIR: &str = "target/release";

/// Executes a command and captures its output
fn execute_command(command: &str, args: &[&str]) -> std::io::Result<String> {
    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Error executing command: {} {}",
            command,
            args.join(" ")
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Checks binary security properties using `readelf` or similar tools
fn check_security(binary_path: &Path) -> std::io::Result<()> {
    println!("Checking security properties for: {}", binary_path.display());

    // Check for stack canary protection
    let readelf_output = execute_command("readelf", &["-s", binary_path.to_str().unwrap()])?;
    if readelf_output.contains("__stack_chk_fail") {
        println!("✔ Stack canary protection enabled in {}", binary_path.display());
    } else {
        println!("✘ Stack canary protection NOT enabled in {}", binary_path.display());
    }

    // Check for ASLR support (position-independent executables)
    let file_output = execute_command("file", &[binary_path.to_str().unwrap()])?;
    if file_output.contains("PIE") {
        println!("✔ ASLR (PIE) enabled in {}", binary_path.display());
    } else {
        println!("✘ ASLR (PIE) NOT enabled in {}", binary_path.display());
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let binaries_dir = Path::new(BINARIES_DIR);

    if !binaries_dir.exists() {
        eprintln!("Binaries directory '{}' does not exist.", BINARIES_DIR);
        return Ok(());
    }

    // Iterate through all files in the binaries directory
    for entry in fs::read_dir(binaries_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            check_security(&path)?;
        }
    }

    println!("Security checks completed for all binaries.");
    Ok(())
}
