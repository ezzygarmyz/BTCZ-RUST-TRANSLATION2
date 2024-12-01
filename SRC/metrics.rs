use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::{info, error};

/// Represents a metric value, which can be numeric or textual
#[derive(Debug, Clone)]
pub enum MetricValue {
    Integer(i64),
    Float(f64),
    Text(String),
}

/// Stores and manages performance metrics
pub struct Metrics {
    metrics: Arc<Mutex<HashMap<String, MetricValue>>>,
    start_time: Instant,
}

impl Metrics {
    /// Creates a new metrics store
    pub fn new() -> Self {
        Metrics {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            start_time: Instant::now(),
        }
    }

    /// Sets a numeric metric (integer)
    pub fn set_integer(&self, key: &str, value: i64) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(key.to_string(), MetricValue::Integer(value));
    }

    /// Sets a numeric metric (float)
    pub fn set_float(&self, key: &str, value: f64) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(key.to_string(), MetricValue::Float(value));
    }

    /// Sets a text metric
    pub fn set_text(&self, key: &str, value: &str) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(key.to_string(), MetricValue::Text(value.to_string()));
    }

    /// Gets the value of a metric by key
    pub fn get_metric(&self, key: &str) -> Option<MetricValue> {
        let metrics = self.metrics.lock().unwrap();
        metrics.get(key).cloned()
    }

    /// Logs all metrics
    pub fn log_metrics(&self) {
        let metrics = self.metrics.lock().unwrap();
        for (key, value) in metrics.iter() {
            info!("Metric - {}: {:?}", key, value);
        }
    }

    /// Returns the uptime of the node in seconds
    pub fn get_uptime(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

/// Periodically logs metrics to the console or a file
pub fn start_metrics_reporting(metrics: Arc<Metrics>, interval: Duration) {
    std::thread::spawn(move || loop {
        std::thread::sleep(interval);

        // Log metrics
        info!("=== Metrics Report ===");
        metrics.log_metrics();
    });
}
