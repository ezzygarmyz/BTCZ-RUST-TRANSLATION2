use std::sync::{Arc, Mutex};

/// Represents the severity level of a warning
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarningLevel {
    Info,
    Warning,
    Critical,
}

/// Warning manager for handling system-wide warnings
pub struct WarningManager {
    warnings: Arc<Mutex<Vec<(WarningLevel, String)>>>,
}

impl WarningManager {
    /// Creates a new WarningManager
    pub fn new() -> Self {
        WarningManager {
            warnings: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds a new warning
    pub fn add_warning(&self, level: WarningLevel, message: &str) {
        let mut warnings = self.warnings.lock().unwrap();
        warnings.push((level, message.to_string()));
    }

    /// Retrieves all warnings
    pub fn get_warnings(&self) -> Vec<(WarningLevel, String)> {
        let warnings = self.warnings.lock().unwrap();
        warnings.clone()
    }

    /// Clears all warnings
    pub fn clear_warnings(&self) {
        let mut warnings = self.warnings.lock().unwrap();
        warnings.clear();
    }

    /// Logs all warnings to the console (or log file)
    pub fn log_warnings(&self) {
        let warnings = self.get_warnings();
        for (level, message) in warnings {
            match level {
                WarningLevel::Info => println!("[INFO]: {}", message),
                WarningLevel::Warning => println!("[WARNING]: {}", message),
                WarningLevel::Critical => eprintln!("[CRITICAL]: {}", message),
            }
        }
    }
}
