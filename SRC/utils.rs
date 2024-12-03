use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::collections::HashMap;
use std::sync::{Mutex, Once};

/// Logging levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Global logger singleton
pub struct Logger {
    level: LogLevel,
    log_file: Option<Mutex<File>>,
}

impl Logger {
    /// Creates a new logger
    pub fn new(level: LogLevel, log_file_path: Option<&Path>) -> Self {
        let log_file = log_file_path
            .map(|path| Mutex::new(File::create(path).unwrap()));

        Logger { level, log_file }
    }

    /// Logs a message at a specific level
    pub fn log(&self, level: LogLevel, message: &str) {
        if level as u8 >= self.level as u8 {
            let log_entry = format!("[{:?}] {}\n", level, message);

            // Write to log file if configured
            if let Some(file) = &self.log_file {
                let mut file = file.lock().unwrap();
                file.write_all(log_entry.as_bytes()).unwrap();
            } else {
                // Print to stdout by default
                print!("{}", log_entry);
            }
        }
    }
}

/// Global logger instance
static mut LOGGER: Option<Logger> = None;
static LOGGER_INIT: Once = Once::new();

/// Initializes the global logger
pub fn init_logger(level: LogLevel, log_file_path: Option<&str>) {
    LOGGER_INIT.call_once(|| {
        unsafe {
            LOGGER = Some(Logger::new(
                level,
                log_file_path.map(Path::new),
            ));
        }
    });
}

/// Logs a message using the global logger
pub fn log(level: LogLevel, message: &str) {
    unsafe {
        if let Some(logger) = &LOGGER {
            logger.log(level, message);
        }
    }
}

/// Reads a configuration file into a key-value map
pub fn read_config(file_path: &str) -> io::Result<HashMap<String, String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut config = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(config)
}
