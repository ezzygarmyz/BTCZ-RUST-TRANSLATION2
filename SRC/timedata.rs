use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// Maximum allowed time offset (70 minutes)
const MAX_TIME_OFFSET: i64 = 70 * 60;

/// Time data structure for managing network time synchronization
pub struct TimeData {
    time_offsets: Mutex<VecDeque<i64>>,
}

impl TimeData {
    /// Creates a new TimeData instance
    pub fn new() -> Self {
        TimeData {
            time_offsets: Mutex::new(VecDeque::new()),
        }
    }

    /// Adds a time offset received from a peer
    pub fn add_time_offset(&self, offset: i64) {
        let mut offsets = self.time_offsets.lock().unwrap();

        // Add the new offset and limit the size
        offsets.push_back(offset);
        if offsets.len() > 200 {
            offsets.pop_front();
        }
    }

    /// Calculates the adjusted time
    pub fn get_adjusted_time(&self) -> i64 {
        let local_time = Self::get_system_time();
        let offsets = self.time_offsets.lock().unwrap();

        if offsets.is_empty() {
            return local_time;
        }

        let mut sorted_offsets: Vec<_> = offsets.iter().cloned().collect();
        sorted_offsets.sort();

        // Median offset
        let median_offset = sorted_offsets[sorted_offsets.len() / 2];
        local_time + median_offset
    }

    /// Validates the time offset against the maximum allowed offset
    pub fn is_time_offset_valid(offset: i64) -> bool {
        offset.abs() <= MAX_TIME_OFFSET
    }

    /// Returns the current system time in seconds since the UNIX epoch
    pub fn get_system_time() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before UNIX epoch")
            .as_secs() as i64
    }
}
