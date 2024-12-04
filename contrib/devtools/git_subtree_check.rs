use std::process::{Command, Stdio};
use std::io;

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

/// Verifies a Git subtree's integrity
fn verify_subtree(path: &str, expected_commit: &str) -> io::Result<()> {
    println!("Verifying subtree at path: {}", path);

    // Fetch the subtree's history
    let subtree_history = execute_command("git", &["log", "--oneline", path])?;
    println!("Subtree history:\n{}", subtree_history);

    // Check if the expected commit exists in the history
    if subtree_history.contains(expected_commit) {
        println!("Subtree at {} is valid. Commit {} found.", path, expected_commit);
    } else {
        eprintln!(
            "Error: Subtree at {} is invalid. Commit {} not found.",
            path, expected_commit
        );
        std::process::exit(1);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Example subtree paths and expected commits
    let subtrees = vec![
        ("subtree/path1", "abcdef1234567890abcdef1234567890abcdef12"),
        ("subtree/path2", "123456abcdef1234567890abcdef1234567890ab"),
    ];

    for (path, commit) in subtrees {
        verify_subtree(path, commit)?;
    }

    println!("All subtrees verified successfully.");
    Ok(())
}
