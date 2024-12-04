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

/// Prompts the user for confirmation
fn prompt_confirmation(prompt: &str) -> bool {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Merges a GitHub pull request
fn merge_pull_request(pr_number: &str) -> io::Result<()> {
    // Check out the PR locally
    println!("Checking out PR #{}...", pr_number);
    execute_command("git", &["fetch", "origin", &format!("pull/{}/head:pr-{}", pr_number, pr_number)])?;
    execute_command("git", &["checkout", &format!("pr-{}", pr_number)])?;

    // Verify the commits are signed
    println!("Verifying commit signatures...");
    let verification_output = execute_command("git", &["log", "--show-signature", "-n", "1"])?;
    println!("{}", verification_output);

    if !prompt_confirmation("Do you want to continue with the merge?") {
        println!("Merge aborted.");
        return Ok(());
    }

    // Merge the PR into the base branch
    println!("Merging PR #{} into the base branch...", pr_number);
    execute_command("git", &["checkout", "main"])?;
    execute_command("git", &["merge", "--squash", &format!("pr-{}", pr_number)])?;

    // Commit the merge
    println!("Committing the merge...");
    execute_command("git", &["commit"])?;

    // Push the changes
    println!("Pushing the changes to the repository...");
    execute_command("git", &["push", "origin", "main"])?;

    println!("Pull request #{} merged successfully!", pr_number);
    Ok(())
}

fn main() -> io::Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: github_merge <PR_NUMBER>");
        std::process::exit(1);
    }

    let pr_number = &args[1];

    // Merge the pull request
    merge_pull_request(pr_number)
}
