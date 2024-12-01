use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Manages experimental features.
pub struct ExperimentalFeatures {
    features: Arc<RwLock<HashMap<String, bool>>>,
}

impl ExperimentalFeatures {
    /// Creates a new instance of ExperimentalFeatures.
    pub fn new() -> Self {
        ExperimentalFeatures {
            features: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Enables a specific experimental feature.
    pub fn enable_feature(&self, feature: &str) {
        let mut features = self.features.write().unwrap();
        features.insert(feature.to_string(), true);
    }

    /// Disables a specific experimental feature.
    pub fn disable_feature(&self, feature: &str) {
        let mut features = self.features.write().unwrap();
        features.insert(feature.to_string(), false);
    }

    /// Checks if a specific experimental feature is enabled.
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        let features = self.features.read().unwrap();
        *features.get(feature).unwrap_or(&false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experimental_features() {
        let features = ExperimentalFeatures::new();

        // Enable a feature
        features.enable_feature("new_algo");
        assert!(features.is_feature_enabled("new_algo"));

        // Disable the feature
        features.disable_feature("new_algo");
        assert!(!features.is_feature_enabled("new_algo"));

        // Check an undefined feature
        assert!(!features.is_feature_enabled("undefined_feature"));
    }
}
