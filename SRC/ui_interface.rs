use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Type alias for UI event callbacks
type UiCallback = Box<dyn Fn(&str) + Send + Sync>;

/// UI Interface for handling notifications and callbacks
pub struct UiInterface {
    callbacks: Mutex<HashMap<String, Vec<UiCallback>>>,
}

impl UiInterface {
    /// Creates a new UI Interface
    pub fn new() -> Self {
        UiInterface {
            callbacks: Mutex::new(HashMap::new()),
        }
    }

    /// Registers a callback for a specific event
    pub fn register_callback<F>(&self, event: &str, callback: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks
            .entry(event.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    /// Unregisters all callbacks for a specific event
    pub fn unregister_callbacks(&self, event: &str) {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.remove(event);
    }

    /// Triggers a UI event with a message
    pub fn trigger_event(&self, event: &str, message: &str) {
        if let Some(callbacks) = self.callbacks.lock().unwrap().get(event) {
            for callback in callbacks {
                callback(message);
            }
        }
    }
}
