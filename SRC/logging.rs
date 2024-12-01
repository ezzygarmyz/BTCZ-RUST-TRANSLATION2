use fern::Dispatch;
use log::{debug, error, info, trace, warn};
use std::fs;
use std::path::Path;

/// Sets up logging with specified log level and optional file output.
pub fn setup_logger(log_file: Option<&Path>, log_level: log::LevelFilter) -> Result<(), fern::InitError> {
    let mut base_config = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout());

    if let Some(log_file) = log_file {
        // Ensure the log directory exists
        if let Some(parent) = log_file.parent() {
            fs::create_dir_all(parent)?;
        }

        base_config = base_config.chain(fern::log_file(log_file)?);
    }

    base_config.apply()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_logging_to_console() {
        setup_logger(None, LevelFilter::Info).unwrap();
        info!("This is an info message.");
        warn!("This is a warning message.");
        error!("This is an error message.");
        debug!("This debug message should not appear.");
    }

    #[test]
    fn test_logging_to_file() {
        let mut temp_dir = PathBuf::from(env::temp_dir());
        temp_dir.push("logging_test.log");

        setup_logger(Some(&temp_dir), LevelFilter::Debug).unwrap();
        info!("Logging to a file.");
        warn!("This warning will also be logged.");
        debug!("Debug message written to file.");

        assert!(temp_dir.exists());
        fs::remove_file(temp_dir).unwrap(); // Clean up
    }
}
