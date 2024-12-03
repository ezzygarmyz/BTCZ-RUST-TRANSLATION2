use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Represents a mockable time system
pub struct TimeManager {
    adjusted_time: Arc<Mutex<Option<i64>>>, // Mocked adjusted time
}

impl TimeManager {
    /// Creates a new TimeManager
    pub fn new() -> Self {
        TimeManager {
            adjusted_time: Arc::new(Mutex::new(None)),
        }
    }

    /// Gets the current system time in seconds since the UNIX epoch
    pub fn current_time(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    /// Gets the adjusted time, falling back to the system time if not mocked
    pub fn adjusted_time(&self) -> i64 {
        let adjusted_time = self.adjusted_time.lock().unwrap();
        adjusted_time.unwrap_or_else(|| self.current_time())
    }

    /// Sets a mocked adjusted time (for testing purposes)
    pub fn set_mock_time(&self, mock_time: i64) {
        let mut adjusted_time = self.adjusted_time.lock().unwrap();
        *adjusted_time = Some(mock_time);
    }

    /// Clears the mocked adjusted time
    pub fn clear_mock_time(&self) {
        let mut adjusted_time = self.adjusted_time.lock().unwrap();
        *adjusted_time = None;
    }
}
