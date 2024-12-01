use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Represents a Univalue-like structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct Univalue {
    pub key: String,
    pub value: Value,
}

impl Univalue {
    /// Create a new Univalue object.
    pub fn new(key: &str, value: Value) -> Self {
        Univalue {
            key: key.to_string(),
            value,
        }
    }

    /// Read a JSON string and parse it into a Univalue object.
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let value: Value = serde_json::from_str(json_str)?;
        Ok(Univalue {
            key: "root".to_string(),
            value,
        })
    }

    /// Serialize the Univalue object into a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.value)
    }

    /// Access a specific key within the Univalue object.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.value.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_univalue_creation() {
        let uni = Univalue::new("example", json!({ "key": "value" }));
        assert_eq!(uni.key, "example");
        assert_eq!(uni.get("key"), Some(&json!("value")));
    }

    #[test]
    fn test_univalue_json() {
        let uni = Univalue::new("test", json!({ "test_key": "test_value" }));
        let json_str = uni.to_json().unwrap();
        assert!(json_str.contains("test_key"));
        assert!(json_str.contains("test_value"));
    }

    #[test]
    fn test_univalue_from_json() {
        let json_str = r#"{ "example_key": "example_value" }"#;
        let uni = Univalue::from_json(json_str).unwrap();
        assert_eq!(uni.get("example_key"), Some(&json!("example_value")));
    }
}
