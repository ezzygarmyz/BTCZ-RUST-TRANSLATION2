use std::process::{Command, Stdio};
use std::env;
use std::io::{self, Write};

/// Executes a shell command and captures its output
fn execute_command(command: &str, args: &[&str]) -> io::Result<String> {
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
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Applies `rustfmt` to specific lines in a file
fn format_diff(file: &str, lines: &[usize]) -> io::Result<()> {
    println!("Formatting file: {} on lines: {:?}", file, lines);

    // Rust doesn't have direct line-range formatting in rustfmt,
    // so we format the entire file as a fallback.
    execute_command("rustfmt", &[file])?;
    println!("Formatted {}", file);
    Ok(())
}

/// Parses the diff and extracts the files and line numbers affected
fn parse_diff(diff: &str) -> Vec<(String, Vec<usize>)> {
    let mut results = Vec::new();
    let mut current_file = None;
    let mut current_lines = Vec::new();

    for line in diff.lines() {
        if line.starts_with("+++ b/") {
            // New file detected; save the previous file
            if let Some(file) = current_file.take() {
                results.push((file, current_lines.clone()));
                current_lines.clear();
            }

            // Extract the file name
            current_file = Some(line.trim_start_matches("+++ b/").to_string());
        } else if line.starts_with("@@") {
            // Extract line numbers from the diff hunk header
            if let Some(start_line) = line.split_whitespace().nth(1) {
                let start_line = start_line
                    .trim_start_matches('+')
                    .split(',')
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap_or(0);
                current_lines.push(start_line);
            }
        }
    }

    // Save the last file
    if let Some(file) = current_file {
        results.push((file, current_lines));
    }

    results
}

fn main() -> io::Result<()> {
    // Parse the input diff from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let diffs = parse_diff(&input);

    for (file, lines) in diffs {
        format_diff(&file, &lines)?;
    }

    Ok(())
}
