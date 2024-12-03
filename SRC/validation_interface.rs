use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Represents a blockchain validation event
pub enum ValidationEvent {
    NewBlock(String),         // Triggered when a new block is added (block hash)
    MempoolUpdate(String),    // Triggered when a transaction is added to the mempool (txid)
    BlockDisconnected(String), // Triggered when a block is disconnected (block hash)
}

/// Type alias for validation callbacks
type ValidationCallback = Box<dyn Fn(&ValidationEvent) + Send + Sync>;

/// Validation interface for registering and triggering events
pub struct ValidationInterface {
    callbacks: Mutex<HashMap<String, Vec<ValidationCallback>>>, // Event -> List of callbacks
}

impl ValidationInterface {
    /// Creates a new ValidationInterface
    pub fn new() -> Self {
        ValidationInterface {
            callbacks: Mutex::new(HashMap::new()),
        }
    }

    /// Registers a callback for a specific event
    pub fn register_callback<F>(&self, event: &str, callback: F)
    where
        F: Fn(&ValidationEvent) + Send + Sync + 'static,
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

    /// Triggers an event and notifies all registered listeners
    pub fn trigger_event(&self, event_type: &str, event: ValidationEvent) {
        if let Some(callbacks) = self.callbacks.lock().unwrap().get(event_type) {
            for callback in callbacks {
                callback(&event);
            }
        }
    }
}
