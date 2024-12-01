#[cfg(test)]
mod tests {
    use crate::metrics::Metrics;

    #[test]
    fn track_metrics() {
        let mut metrics = Metrics::new();
        metrics.increment("transactions");
        assert_eq!(metrics.get("transactions"), 1);
    }
}

pub mod metrics {
    use std::collections::HashMap;

    pub struct Metrics {
        counters: HashMap<String, usize>,
    }

    impl Metrics {
        pub fn new() -> Self {
            Metrics {
                counters: HashMap::new(),
            }
        }

        pub fn increment(&mut self, key: &str) {
            *self.counters.entry(key.to_string()).or_insert(0) += 1;
        }

        pub fn get(&self, key: &str) -> usize {
            *self.counters.get(key).unwrap_or(&0)
        }
    }
}
