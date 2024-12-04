use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Command-line tools and their descriptions
const TOOLS: &[(&str, &str)] = &[
    ("bitcoinz-cli", "Command-line interface for interacting with the BitcoinZ daemon"),
    ("bitcoind", "The BitcoinZ daemon that processes blocks and transactions"),
];

/// Generates a manpage for a given tool
fn generate_manpage(tool_name: &str, description: &str) -> io::Result<()> {
    let output_dir = "manpages";
    fs::create_dir_all(output_dir)?;

    let manpage_path = format!("{}/{}.1", output_dir, tool_name);
    let mut manpage_file = File::create(&manpage_path)?;

    let content = format!(
        ".TH {} 1 \"{}\" \"BitcoinZ\" \"User Commands\"\n\
        .SH NAME\n\
        {} - {}\n\
        .SH SYNOPSIS\n\
        {} [OPTIONS]\n\
        .SH DESCRIPTION\n\
        {}\n",
        tool_name,
        chrono::Utc::now().format("%Y-%m-%d"),
        tool_name,
        description,
        tool_name,
        description,
    );

    manpage_file.write_all(content.as_bytes())?;
    println!("Generated manpage for {} at {}", tool_name, manpage_path);
    Ok(())
}

/// Compresses the generated manpages using gzip
fn compress_manpages() -> io::Result<()> {
    let output_dir = "manpages";

    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "1" {
            let output = Command::new("gzip")
                .arg("-f")
                .arg(path.to_str().unwrap())
                .stdout(Stdio::inherit())
                .output()?;

            if !output.status.success() {
                eprintln!("Failed to compress {:?}", path);
            }
        }
    }

    println!("Manpages compressed successfully.");
    Ok(())
}

fn main() -> io::Result<()> {
    // Ensure required tools are available
    if Command::new("gzip").output().is_err() {
        eprintln!("Error: gzip command is required but not found.");
        return Ok(());
    }

    // Generate manpages for all tools
    for (tool, description) in TOOLS {
        generate_manpage(tool, description)?;
    }

    // Compress the manpages
    compress_manpages()?;

    println!("All manpages generated and compressed successfully.");
    Ok(())
}
