use log::{Level, LevelFilter, Metadata, Record};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

/// Configuration for the logger
pub struct LoggerConfig {
    pub log_to_console: bool,
    pub log_to_file: bool,
    pub file_path: Option<String>,
    pub verbosity: LevelFilter,
}

/// Custom logger implementation
pub struct CustomLogger {
    config: Arc<Mutex<LoggerConfig>>,
    file_handle: Arc<Mutex<Option<io::BufWriter<std::fs::File>>>>,
}

impl CustomLogger {
    /// Creates a new logger instance with the given configuration
    pub fn new(config: LoggerConfig) -> Self {
        let file_handle = if config.log_to_file {
            if let Some(ref path) = config.file_path {
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .ok();
                Some(io::BufWriter::new(file.unwrap()))
            } else {
                None
            }
        } else {
            None
        };

        Self {
            config: Arc::new(Mutex::new(config)),
            file_handle: Arc::new(Mutex::new(file_handle)),
        }
    }

    /// Logs a message
    fn log(&self, level: Level, message: &str) {
        let config = self.config.lock().unwrap();
        if level <= config.verbosity.to_level().unwrap_or(Level::Info) {
            let formatted_message = format!("[{}] {}\n", level, message);

            // Log to console
            if config.log_to_console {
                print!("{}", formatted_message);
            }

            // Log to file
            if let Some(ref mut file) = *self.file_handle.lock().unwrap() {
                let _ = file.write_all(formatted_message.as_bytes());
                let _ = file.flush();
            }
        }
    }

    /// Updates the logger configuration
    pub fn update_config(&self, new_config: LoggerConfig) {
        let mut config = self.config.lock().unwrap();
        *config = new_config;

        // Reinitialize file handle if file logging is enabled
        if config.log_to_file {
            if let Some(ref path) = config.file_path {
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .ok();
                *self.file_handle.lock().unwrap() = Some(io::BufWriter::new(file.unwrap()));
            }
        }
    }
}

/// Logger trait implementation for the log crate
impl log::Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let config = self.config.lock().unwrap();
        metadata.level() <= config.verbosity.to_level().unwrap_or(Level::Info)
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.log(record.level(), &record.args().to_string());
        }
    }

    fn flush(&self) {}
}

/// Initializes the custom logger with the given configuration
pub fn init_logger(config: LoggerConfig) -> Result<(), log::SetLoggerError> {
    let logger = CustomLogger::new(config);
    log::set_max_level(logger.config.lock().unwrap().verbosity);
    log::set_boxed_logger(Box::new(logger))
}
